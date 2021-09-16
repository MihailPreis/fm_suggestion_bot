use teloxide::prelude::*;
extern crate dotenv;

use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use teloxide::RequestError;
use tokio_stream::wrappers::UnboundedReceiverStream;

lazy_static! {
    static ref CHANNEL_ID: String = std::env::var("CHANNEL_ID").unwrap().to_string();
    static ref ADMINS_CHAT_ID: String = std::env::var("ADMINS_CHAT_ID").unwrap().to_string();
}

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    dotenv().ok();
    let bot = Bot::from_env();
    log::info!("Bot is running.");
    Dispatcher::new(bot)
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
    let accept_button =
        InlineKeyboardButton::callback("✅ Accept".to_string(), "accept".to_string());
    let accept_without_text_button = InlineKeyboardButton::callback(
        "☢️ Without text".to_string(),
        "accept-without-text".to_string(),
    );
    let decline_button =
        InlineKeyboardButton::callback("❌ Decline".to_string(), "decline".to_string());
    let keyboard = InlineKeyboardMarkup::default()
        .append_row(vec![accept_button, accept_without_text_button])
        .append_row(vec![decline_button]);
    let _mes = cx.forward_to(ADMINS_CHAT_ID.to_string()).send().await?;
    let user = cx.update.from().ok_or(RequestError::RetryAfter(0))?;
    cx.requester
        .send_message(
            ADMINS_CHAT_ID.to_string(),
            format!(
                "From: <@{}> {} {}\nWe going to shitpost it?",
                user.username.as_ref().unwrap_or(&String::new()),
                user.first_name,
                user.last_name.as_ref().unwrap_or(&String::new()),
            ),
        )
        .reply_to_message_id(_mes.id)
        .reply_markup(keyboard)
        .send()
        .await?;
    Ok(())
}

async fn callback_handler(cx: UpdateWithCx<Bot, CallbackQuery>) -> Result<(), RequestError> {
    let data = cx.update.data.clone().ok_or(RequestError::RetryAfter(0))?;
    let message = cx.update.message.ok_or(RequestError::RetryAfter(0))?;
    if data.starts_with("accept") {
        let id = message
            .reply_to_message()
            .ok_or(RequestError::RetryAfter(0))?
            .id;
        let _mes = cx
            .requester
            .copy_message(CHANNEL_ID.to_string(), message.chat_id(), id)
            .send()
            .await?;
        if data.starts_with("accept-without-text") {
            cx.requester
                .edit_message_text(CHANNEL_ID.to_string(), _mes.id, "")
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
