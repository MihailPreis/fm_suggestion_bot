extern crate dotenv;

use std::env;

use dotenv::dotenv;
use lazy_static::lazy_static;
use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use teloxide::RequestError;
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::utils::message_utils::ExtMessage;
use crate::utils::result_utils::FatalValueMapper;
use crate::utils::user_utils::ExtUser;

mod utils;

static CHANNEL_ID_KEY: &str = "CHANNEL_ID";
static ADMINS_CHAT_ID_KEY: &str = "ADMINS_CHAT_ID";
static TELOXIDE_TOKEN_KEY: &str = "TELOXIDE_TOKEN";

static ACCEPT_CALLBACK: &str = "accept";
static DECLINE_CALLBACK: &str = "decline";
static WITHOUT_TEXT_CALLBACK: &str = "accept-without-text";

fn get_env_key(key: &str) -> String {
    env::var(key).map_value_or_exit(format!("Can not get value for key {}, exiting", key))
}

lazy_static! {
    static ref CHANNEL_ID: String = get_env_key(CHANNEL_ID_KEY);
    static ref ADMINS_CHAT_ID: String = get_env_key(ADMINS_CHAT_ID_KEY);
    static ref TELOXIDE_TOKEN: String = get_env_key(TELOXIDE_TOKEN_KEY);
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    teloxide::enable_logging!();
    log::info!("Bot is running.");
    Dispatcher::new(Bot::new(TELOXIDE_TOKEN.to_string()))
        .messages_handler(|rx: DispatcherHandlerRx<Bot, Message>| {
            UnboundedReceiverStream::new(rx).for_each_concurrent(None, |cx| async move {
                match message_handler(cx).await {
                    Ok(_) => {}
                    Err(e) => log::warn!("{}", e),
                }
            })
        })
        .callback_queries_handler(|rx: DispatcherHandlerRx<Bot, CallbackQuery>| {
            UnboundedReceiverStream::new(rx).for_each_concurrent(None, |cx| async move {
                match callback_handler(cx).await {
                    Ok(_) => {}
                    Err(e) => log::warn!("{}", e),
                }
            })
        })
        .dispatch()
        .await;
}

async fn message_handler(cx: UpdateWithCx<Bot, Message>) -> Result<(), RequestError> {
    if cx.update.chat.id.to_string() == ADMINS_CHAT_ID.to_string() {
        return Ok(());
    }

    let _mes = cx.forward_to(ADMINS_CHAT_ID.to_string()).send().await?;
    let user = cx.update.from().ok_or(RequestError::RetryAfter(0))?;
    cx.requester
        .send_message(
            ADMINS_CHAT_ID.to_string(),
            format!("From: {}\nWe going to shitpost it?", user.ftm_title(),),
        )
        .reply_to_message_id(_mes.id)
        .reply_markup(build_keyboard(cx.update.has_caption()))
        .send()
        .await?;

    Ok(())
}

async fn callback_handler(cx: UpdateWithCx<Bot, CallbackQuery>) -> Result<(), RequestError> {
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
