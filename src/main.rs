extern crate dotenv;

use std::borrow::Borrow;
use std::env;

use dotenv::dotenv;
use lazy_static::lazy_static;
use teloxide::prelude::*;
use teloxide::types::{ChatId, InlineKeyboardButton, InlineKeyboardMarkup, InputFile};
use teloxide::RequestError;
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::data::db::{create_database_if_needed, migrate};
use crate::data::model::offered_post::OfferedPost;
use crate::data::repo::offered_post_repo::OfferedPostRepo;
use crate::utils::env_utils::get_env_key;
use crate::utils::message_utils::ExtMessage;
use crate::utils::pic_utils::get_pic;
use crate::utils::user_utils::ExtUser;

mod data;
mod utils;

static CHANNEL_ID_KEY: &str = "CHANNEL_ID";
static ADMINS_CHAT_ID_KEY: &str = "ADMINS_CHAT_ID";
static TELOXIDE_TOKEN_KEY: &str = "TELOXIDE_TOKEN";

static ACCEPT_CALLBACK: &str = "accept";
static DECLINE_CALLBACK: &str = "decline";
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
    migrate().await;
    log::info!("Bot is running.");
    Dispatcher::new(Bot::new(TELOXIDE_TOKEN.to_string()))
        .messages_handler(|rx: DispatcherHandlerRx<Bot, Message>| {
            UnboundedReceiverStream::new(rx).for_each_concurrent(None, |cx| async move {
                let offered_post_repo = OfferedPostRepo::new().await;
                match message_handler(cx, offered_post_repo.borrow()).await {
                    Ok(_) => {}
                    Err(e) => log::warn!("{}", e),
                }
            })
        })
        .callback_queries_handler(|rx: DispatcherHandlerRx<Bot, CallbackQuery>| {
            UnboundedReceiverStream::new(rx).for_each_concurrent(None, |cx| async move {
                let offered_post_repo = OfferedPostRepo::new().await;
                match callback_handler(cx, &offered_post_repo).await {
                    Ok(_) => {}
                    Err(e) => log::warn!("{}", e),
                }
            })
        })
        .dispatch()
        .await;
}

async fn message_handler(
    cx: UpdateWithCx<Bot, Message>,
    offered_post_repo: &OfferedPostRepo,
) -> Result<(), RequestError> {
    if cx.update.chat.id.to_string() == ADMINS_CHAT_ID.to_string() {
        return Ok(());
    }

    let _mes = cx.forward_to(ADMINS_CHAT_ID.to_string()).send().await?;
    let user = cx.update.from().ok_or(RequestError::RetryAfter(0))?;
    let message = cx
        .requester
        .send_message(
            ADMINS_CHAT_ID.to_string(),
            format!("From: {}\nWe going to shitpost it?", user.ftm_title(),),
        )
        .reply_to_message_id(_mes.id)
        .reply_markup(build_keyboard(cx.update.has_caption()))
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
) -> Result<(), RequestError> {
    let data = cx.update.data.clone().ok_or(RequestError::RetryAfter(0))?;
    let message = cx.update.message.ok_or(RequestError::RetryAfter(0))?;
    let origin = message
        .reply_to_message()
        .ok_or(RequestError::RetryAfter(0))?;
    if data.starts_with(ACCEPT_CALLBACK) {
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
    }
    let offered_post = offered_post_repo
        .get_offered_post(message.chat_id(), message.id)
        .await;
    match offered_post {
        Ok(post) => {
            match get_pic(data.starts_with(ACCEPT_CALLBACK)) {
                None => {
                    cx.requester
                        .send_message(
                            ChatId::Id(post.chat_id),
                            if data.starts_with(ACCEPT_CALLBACK) {
                                "🎉 Post is published."
                            } else {
                                "🚧 Post was rejected. Send me something cooler."
                            },
                        )
                        .reply_to_message_id(post.message_id)
                        .send()
                        .await?
                }
                Some(pic) => {
                    cx.requester
                        .send_video(ChatId::Id(post.chat_id), InputFile::memory("file.mp4", pic))
                        .reply_to_message_id(post.message_id)
                        .send()
                        .await?
                }
            };
        }
        Err(_) => {}
    }
    cx.requester
        .delete_message(message.chat_id(), message.id)
        .send()
        .await?;
    Ok(())
}

fn build_keyboard(has_caption: bool) -> InlineKeyboardMarkup {
    let accept_button =
        InlineKeyboardButton::callback("✅ Accept".to_string(), ACCEPT_CALLBACK.to_string());
    let decline_button =
        InlineKeyboardButton::callback("❌ Decline".to_string(), DECLINE_CALLBACK.to_string());
    if has_caption {
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
