use std::borrow::Cow;
use std::env;

use lazy_static::lazy_static;
use regex::Regex;
use teloxide::prelude::*;
use teloxide::types::ParseMode::MarkdownV2;
use teloxide::types::{ChatId, InputFile};

use crate::data::model::pic::Pic;
use crate::data::repo::offered_post_repo::OfferedPostRepo;
use crate::data::repo::pic_repo::PicRepo;
use crate::utils::document_utils::download_animate_vec;
use crate::utils::error_utils::HandlerError;
use crate::utils::option_utils::unwrap_send_error;
use crate::utils::version::VERSION_STRING;

static HELP_CMD: &str = "/help";
static VERSION_CMD: &str = "/version";
static LIST_CMD: &str = "/list";
static GET_CMD: &str = "/get";
static DELETE_CMD: &str = "/rm";
static ADD_CMD: &str = "/add";
static MSG_CMD: &str = "/msg";

static MSG_PREFIX_KEY: &str = "MSG_PREFIX";

lazy_static! {
    static ref GET_REGEX: Regex = Regex::new(r"/get (A|D) (.+)").unwrap();
    static ref ADD_REGEX: Regex = Regex::new(r"/add (A|D)").unwrap();
    static ref RM_REGEX: Regex = Regex::new(r"/rm (A|D) (.+)").unwrap();
    static ref MSG_REGEX: Regex = Regex::new(r"/msg (.+)").unwrap();
    static ref MSG_PREFIX: String = env::var(MSG_PREFIX_KEY).unwrap_or(String::new());
}

pub async fn exec_command(
    text: &str,
    cx: &UpdateWithCx<Bot, Message>,
    pic_repo: &PicRepo,
    offered_post_repo: &OfferedPostRepo,
) -> Result<(), HandlerError> {
    if text.starts_with(VERSION_CMD) {
        version(cx).await?
    } else if text.starts_with(HELP_CMD) {
        help(cx).await?
    } else if text.starts_with(LIST_CMD) {
        list(cx, pic_repo).await?
    } else if text.starts_with(ADD_CMD) {
        add(&cx, pic_repo, &text).await?
    } else if text.starts_with(GET_CMD) {
        get(cx, pic_repo, text).await?
    } else if text.starts_with(DELETE_CMD) {
        delete(cx, pic_repo, text).await?
    } else if text.starts_with(MSG_CMD) {
        send_msg(cx, offered_post_repo, text).await?
    }
    Ok(())
}

async fn delete(
    cx: &UpdateWithCx<Bot, Message>,
    pic_repo: &PicRepo,
    text: &str,
) -> Result<(), HandlerError> {
    let captures = unwrap_send_error(
        RM_REGEX.captures(text),
        cx,
        "Invalid parameters for Rm command. See /help",
    )
    .await?;

    let for_accept = captures.get(1).unwrap().as_str() == "A";
    let file_name = captures.get(2).unwrap().as_str();
    if let Err(_) = pic_repo.delete_pic(file_name.to_string(), for_accept).await {
        cx.reply_to("Image with this filename and mark does not exist.")
            .send()
            .await?;
    } else {
        cx.reply_to("Delete successful.").send().await?;
    }
    Ok(())
}

async fn get(
    cx: &UpdateWithCx<Bot, Message>,
    pic_repo: &PicRepo,
    text: &str,
) -> Result<(), HandlerError> {
    let captures = unwrap_send_error(
        GET_REGEX.captures(text),
        cx,
        "Invalid parameters for Get command. See /help",
    )
    .await?;

    let for_accept = captures.get(1).unwrap().as_str() == "A";
    let file_name = captures.get(2).unwrap().as_str();

    let pic = unwrap_send_error(
        pic_repo
            .get_pic(file_name.to_string(), for_accept)
            .await
            .ok(),
        cx,
        "Pic with this name and mark not found. See /list",
    )
    .await?;

    cx.reply_animation(InputFile::Memory {
        file_name: pic.file_name,
        data: Cow::from(pic.data),
    })
    .send()
    .await?;
    Ok(())
}

async fn add(
    cx: &&UpdateWithCx<Bot, Message>,
    pic_repo: &PicRepo,
    text: &&str,
) -> Result<(), HandlerError> {
    let captures = unwrap_send_error(
        ADD_REGEX.captures(&text),
        cx,
        "Invalid parameters for Add command. See /help",
    )
    .await?;
    let animation = unwrap_send_error(
        cx.update.animation(),
        cx,
        "Attach animation to Add command. See /help",
    )
    .await?;
    let data = unwrap_send_error(
        download_animate_vec(animation, &cx.requester).await,
        cx,
        "Download error.",
    )
    .await?;
    let for_accept = captures.get(1).unwrap().as_str() == "A";
    let default_file_name = String::from("file.gif");
    let file_name = animation.file_name.as_ref().unwrap_or(&default_file_name);
    if let Ok(_) = pic_repo.get_pic(file_name.to_string(), for_accept).await {
        cx.reply_to("Pic with this name and mark already exists.")
            .send()
            .await?;
    } else {
        if let Err(_) = pic_repo
            .save_pic(Pic::new(file_name.to_string(), for_accept, data.clone()))
            .await
        {
            cx.reply_to("Add error. Smoke logs.").send().await?;
        } else {
            cx.reply_to("Add successful.").send().await?;
        }
    }
    Ok(())
}

async fn list(cx: &UpdateWithCx<Bot, Message>, pic_repo: &PicRepo) -> Result<(), HandlerError> {
    if let Ok(pics) = pic_repo.get_all_pics().await {
        let _list: String = pics
            .iter()
            .map(|item| {
                format!(
                    "  - {} | {}",
                    if item.for_accept { "A" } else { "D" },
                    item.file_name
                )
            })
            .collect::<Vec<String>>()
            .join("\n");
        if _list.is_empty() {
            cx.reply_to("Pic list is empty ").send().await?;
        } else {
            cx.reply_to(format!("Pic list:\n{}", _list)).send().await?;
        }
    } else {
        cx.reply_to("An error occurred when requesting Pics list. Smoke logs.")
            .send()
            .await?;
    }
    Ok(())
}

async fn help(cx: &UpdateWithCx<Bot, Message>) -> Result<(), HandlerError> {
    cx.reply_to(
        "Bot support next commands:\n\
             - /version - get current version.\n\
             - /list - get all pics from database with mark of accept/decline.\n\
             - /get {A/D} <file_name (from /list)> - get pic.\n\
             - /add {A/D} - add pic.\n\
             - /rm {A/D} <file_name (from /list)> - remove pic.",
    )
    .send()
    .await?;
    Ok(())
}

async fn version(cx: &UpdateWithCx<Bot, Message>) -> Result<(), HandlerError> {
    cx.reply_to(VERSION_STRING).send().await?;
    Ok(())
}

async fn send_msg(
    cx: &UpdateWithCx<Bot, Message>,
    offered_post_repo: &OfferedPostRepo,
    text: &str,
) -> Result<(), HandlerError> {
    let captures = unwrap_send_error(
        MSG_REGEX.captures(text),
        cx,
        "Invalid parameters for Msg command. See /help",
    )
    .await?;
    let message =
        unwrap_send_error(cx.update.reply_to_message(), cx, "Reply message not found.").await?;
    let post = unwrap_send_error(
        offered_post_repo
            .get_offered_post(message.chat_id(), message.id)
            .await
            .ok(),
        cx,
        "Offered post not found.",
    )
    .await?;
    let msg = captures.get(1).unwrap().as_str();

    cx.requester
        .send_message(
            ChatId::Id(post.chat_id),
            format!("{}{}", MSG_PREFIX.as_str(), msg),
        )
        .reply_to_message_id(post.message_id)
        .parse_mode(MarkdownV2)
        .send()
        .await?;

    Ok(())
}
