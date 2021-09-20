extern crate dotenv;

use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env;
use teloxide::prelude::*;
use teloxide::types::{ChatId, InlineKeyboardButton, InlineKeyboardMarkup, InputFile};
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::admin_commands::exec_command;
use crate::data::db::{create_database_if_needed, create_pool, migrate};
use crate::data::model::cached_pic::CachedPic;
use crate::data::model::offered_post::OfferedPost;
use crate::data::repo::cached_pic_repo::CachedPicRepo;
use crate::data::repo::offered_post_repo::OfferedPostRepo;
use crate::data::repo::pic_repo::PicRepo;
use crate::utils::document_utils::download_doc_vec;
use crate::utils::env_utils::get_env_key;
use crate::utils::error_utils::HandlerError;
use crate::utils::message_utils::ExtMessage;
use crate::utils::mime_utils::{is_animate, is_image, is_video};
use crate::utils::pic_utils::{get_pic, GetPicResult};
use crate::utils::user_utils::ExtUser;

mod admin_commands;
mod data;
mod utils;

static CHANNEL_ID_KEY: &str = "CHANNEL_ID";
static ADMINS_CHAT_ID_KEY: &str = "ADMINS_CHAT_ID";
static TELOXIDE_TOKEN_KEY: &str = "TELOXIDE_TOKEN";

static ACCEPT_CALLBACK: &str = "accept";
static DECLINE_CALLBACK: &str = "decline";
static SILENT_DECLINE_CALLBACK: &str = "decline-silent";
static WITHOUT_TEXT_CALLBACK: &str = "accept-without-text";

lazy_static! {
    static ref CHANNEL_ID: String = get_env_key(CHANNEL_ID_KEY);
    static ref ADMINS_CHAT_ID: String = get_env_key(ADMINS_CHAT_ID_KEY);
    static ref TELOXIDE_TOKEN: String = get_env_key(TELOXIDE_TOKEN_KEY);
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    teloxide::enable_logging!();
    create_database_if_needed().await;
    let pool = create_pool().await;
    migrate(&pool).await;
    let offered_post_repo = OfferedPostRepo::new(pool.clone());
    let message_handler_repo = offered_post_repo.clone();
    let queries_handler_repo = offered_post_repo.clone();
    let cached_pic_repo = CachedPicRepo::new(pool.clone());
    let pic_repo = PicRepo::new(pool.clone());
    let message_handler_pic_repo = pic_repo.clone();
    let queries_handler_pic_repo = pic_repo.clone();
    log::info!("Bot is running.");
    Dispatcher::new(Bot::new(TELOXIDE_TOKEN.to_string()))
        .messages_handler(|rx: DispatcherHandlerRx<Bot, Message>| {
            UnboundedReceiverStream::new(rx).for_each_concurrent(None, move |cx| {
                let offered_post_repo = message_handler_repo.clone();
                let pic_repo = message_handler_pic_repo.clone();
                async move {
                    match message_handler(cx, &offered_post_repo, &pic_repo).await {
                        Ok(_) => {}
                        Err(err) => log::warn!("{}", err),
                    }
                }
            })
        })
        .callback_queries_handler(|rx: DispatcherHandlerRx<Bot, CallbackQuery>| {
            UnboundedReceiverStream::new(rx).for_each_concurrent(None, move |cx| {
                let offered_post_repo = queries_handler_repo.clone();
                let cached_pic_repo = cached_pic_repo.clone();
                let pic_repo = queries_handler_pic_repo.clone();
                async move {
                    match callback_handler(cx, &offered_post_repo, &cached_pic_repo, &pic_repo)
                        .await
                    {
                        Ok(_) => {}
                        Err(err) => log::warn!("{}", err),
                    }
                }
            })
        })
        .dispatch()
        .await;
}

async fn message_handler(
    cx: UpdateWithCx<Bot, Message>,
    offered_post_repo: &OfferedPostRepo,
    pic_repo: &PicRepo,
) -> Result<(), HandlerError> {
    if cx.update.chat.id.to_string() == ADMINS_CHAT_ID.to_string() {
        exec_command(&cx, pic_repo).await?;
        return Ok(());
    }
    if let Some(text) = cx.update.text() {
        if text.starts_with("/") {
            return Ok(());
        }
    }

    let _mes = cx.forward_to(ADMINS_CHAT_ID.to_string()).send().await?;
    let user = cx
        .update
        .from()
        .ok_or(HandlerError::from_str("User not found"))?;
    let message = cx
        .requester
        .send_message(
            ADMINS_CHAT_ID.to_string(),
            format!("From: {}\nWe going to shitpost it?", user.ftm_title(),),
        )
        .reply_to_message_id(_mes.id)
        .reply_markup(build_keyboard(
            cx.update.has_caption(),
            cx.update.text().is_some(),
        ))
        .send()
        .await?;
    let _ = offered_post_repo
        .save_offered_post(OfferedPost::new(
            cx.update.chat_id(),
            cx.update.id,
            message.chat.id,
            message.id,
        ))
        .await;
    Ok(())
}

async fn callback_handler(
    cx: UpdateWithCx<Bot, CallbackQuery>,
    offered_post_repo: &OfferedPostRepo,
    cached_pic_repo: &CachedPicRepo,
    pic_repo: &PicRepo,
) -> Result<(), HandlerError> {
    let data = cx
        .update
        .data
        .clone()
        .ok_or(HandlerError::from_str("Data not found"))?;
    let message = cx
        .update
        .message
        .as_ref()
        .ok_or(HandlerError::from_str("Message not found"))?;
    let origin = message
        .reply_to_message()
        .ok_or(HandlerError::from_str("Reply message are missing"))?;
    let is_accept = data.starts_with(ACCEPT_CALLBACK);
    if is_accept {
        if let Some(doc) = origin.document() {
            if is_image(doc) {
                if let Some(image) = download_doc_vec(doc, &cx.requester).await {
                    let r = cx.requester.send_photo(
                        CHANNEL_ID.to_string(),
                        InputFile::memory("image.png", image),
                    );
                    if origin.has_caption() && !data.starts_with(WITHOUT_TEXT_CALLBACK) {
                        r.caption(origin.caption().unwrap().to_string())
                            .send()
                            .await?;
                    } else {
                        r.send().await?;
                    }
                } else {
                    simple_copy(&cx, &data, &message, origin).await?
                }
            } else if is_animate(doc) {
                if let Some(image) = download_doc_vec(doc, &cx.requester).await {
                    let r = cx.requester.send_animation(
                        CHANNEL_ID.to_string(),
                        InputFile::memory("image.gif", image),
                    );
                    if origin.has_caption() && !data.starts_with(WITHOUT_TEXT_CALLBACK) {
                        r.caption(origin.caption().unwrap().to_string())
                            .send()
                            .await?;
                    } else {
                        r.send().await?;
                    }
                } else {
                    simple_copy(&cx, &data, &message, origin).await?
                }
            } else if is_video(doc) {
                if let Some(video) = download_doc_vec(doc, &cx.requester).await {
                    let r = cx.requester.send_video(
                        CHANNEL_ID.to_string(),
                        InputFile::memory("image.mp4", video),
                    );
                    if origin.has_caption() && !data.starts_with(WITHOUT_TEXT_CALLBACK) {
                        r.caption(origin.caption().unwrap().to_string())
                            .send()
                            .await?;
                    } else {
                        r.send().await?;
                    }
                } else {
                    simple_copy(&cx, &data, &message, origin).await?
                }
            } else {
                simple_copy(&cx, &data, &message, origin).await?
            }
        } else {
            simple_copy(&cx, &data, &message, origin).await?
        }
    }
    if !data.starts_with(SILENT_DECLINE_CALLBACK) {
        let offered_post = offered_post_repo
            .get_offered_post(message.chat_id(), message.id)
            .await;
        match offered_post {
            Ok(post) => {
                match get_pic(is_accept, cached_pic_repo, pic_repo).await {
                    None => {
                        cx.requester
                            .send_message(
                                ChatId::Id(post.chat_id),
                                if is_accept {
                                    "🎉 Post is published."
                                } else {
                                    "🚧 Post was rejected. Send me something cooler."
                                },
                            )
                            .reply_to_message_id(post.message_id)
                            .send()
                            .await?
                    }
                    Some(pic) => match pic {
                        GetPicResult::Raw(filename, vector) => {
                            let response: Message = cx
                                .requester
                                .send_animation(
                                    ChatId::Id(post.chat_id),
                                    InputFile::memory(filename.to_string(), vector),
                                )
                                .reply_to_message_id(post.message_id)
                                .send()
                                .await?;
                            if let Some(video) = response.video() {
                                cached_pic_repo
                                    .save_cached_pic(CachedPic {
                                        image_name: filename,
                                        image_file_id: video.file_id.to_string(),
                                    })
                                    .await?;
                            }
                            response
                        }
                        GetPicResult::FileId(file_id) => {
                            cx.requester
                                .send_video(ChatId::Id(post.chat_id), InputFile::file_id(file_id))
                                .reply_to_message_id(post.message_id)
                                .send()
                                .await?
                        }
                    },
                };
            }
            Err(_) => {}
        }
    }
    cx.requester
        .delete_message(message.chat_id(), origin.id)
        .send()
        .await?;
    cx.requester
        .delete_message(message.chat_id(), message.id)
        .send()
        .await?;
    Ok(())
}

async fn simple_copy(
    cx: &UpdateWithCx<Bot, CallbackQuery>,
    data: &String,
    message: &Message,
    origin: &Message,
) -> Result<(), HandlerError> {
    let _mes = cx
        .requester
        .copy_message(CHANNEL_ID.to_string(), message.chat_id(), origin.id)
        .send()
        .await?;
    if data.starts_with(WITHOUT_TEXT_CALLBACK) && origin.has_caption() {
        cx.requester
            .edit_message_caption(CHANNEL_ID.to_string(), _mes.message_id)
            .send()
            .await?;
    }
    Ok(())
}

fn build_keyboard(has_caption: bool, only_text: bool) -> InlineKeyboardMarkup {
    let accept_button =
        InlineKeyboardButton::callback("✅ Accept".to_string(), ACCEPT_CALLBACK.to_string());
    let decline_button =
        InlineKeyboardButton::callback("❌ Decline".to_string(), DECLINE_CALLBACK.to_string());
    let silent_decline_button = InlineKeyboardButton::callback(
        "🗿 Silent decline".to_string(),
        SILENT_DECLINE_CALLBACK.to_string(),
    );
    if only_text {
        InlineKeyboardMarkup::default()
            .append_row(vec![accept_button, decline_button])
            .append_row(vec![silent_decline_button])
    } else if has_caption {
        let accept_without_text_button = InlineKeyboardButton::callback(
            "☢️ Without text".to_string(),
            WITHOUT_TEXT_CALLBACK.to_string(),
        );
        InlineKeyboardMarkup::default()
            .append_row(vec![accept_button, accept_without_text_button])
            .append_row(vec![decline_button])
    } else {
        InlineKeyboardMarkup::default().append_row(vec![accept_button, decline_button])
    }
}
