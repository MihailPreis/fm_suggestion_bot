use crate::data::model::pic::Pic;
use crate::data::repo::pic_repo::PicRepo;
use crate::utils::document_utils::download_animate_vec;
use crate::utils::error_utils::HandlerError;
use crate::utils::version::VERSION_STRING;
use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Cow;
use teloxide::prelude::*;
use teloxide::types::InputFile;

static HELP_CMD: &str = "/help";
static VERSION_CMD: &str = "/version";
static LIST_CMD: &str = "/list";
static GET_CMD: &str = "/get";
static DELETE_CMD: &str = "/rm";
static ADD_CMD: &str = "/add";

lazy_static! {
    static ref GET_REGEX: Regex = Regex::new(r"/get (A|D) (.+)").unwrap();
    static ref ADD_REGEX: Regex = Regex::new(r"/add (A|D)").unwrap();
    static ref RM_REGEX: Regex = Regex::new(r"/rm (A|D) (.+)").unwrap();
}

pub async fn exec_command(
    text: &str,
    cx: &UpdateWithCx<Bot, Message>,
    pic_repo: &PicRepo,
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
    }
    Ok(())
}

async fn delete(
    cx: &UpdateWithCx<Bot, Message>,
    pic_repo: &PicRepo,
    text: &str,
) -> Result<(), HandlerError> {
    let captures = RM_REGEX.captures(text);
    if captures.is_none() {
        cx.reply_to("Invalid parameters. See /help").send().await?;
        return Err(HandlerError::from_str("Invalid parameters for Rm command."));
    }
    let captures = captures.unwrap();

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
    let captures = GET_REGEX.captures(text);
    if captures.is_none() {
        cx.reply_to("Invalid parameters. See /help").send().await?;
        return Err(HandlerError::from_str(
            "Invalid parameters for Get command.",
        ));
    }
    let captures = captures.unwrap();

    let for_accept = captures.get(1).unwrap().as_str() == "A";
    let file_name = captures.get(2).unwrap().as_str();

    let pic = pic_repo.get_pic(file_name.to_string(), for_accept).await;
    if pic.is_err() {
        cx.reply_to("Pic with this name and mark not found. See /list")
            .send()
            .await?;
        return Err(HandlerError::from_str(
            "Pic with this name and mark not found.",
        ));
    }
    let pic = pic.unwrap();

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
    let captures = ADD_REGEX.captures(&text);
    if captures.is_none() {
        cx.reply_to("Invalid parameters. See /help").send().await?;
        return Err(HandlerError::from_str(
            "Invalid parameters for Add command.",
        ));
    }
    let captures = captures.unwrap();

    let animation = cx.update.animation();
    if animation.is_none() {
        cx.reply_to("Attach animation to command. See /help")
            .send()
            .await?;
        return Err(HandlerError::from_str(
            "Invalid parameters for Add command.",
        ));
    }
    let animation = animation.unwrap();

    let data = download_animate_vec(animation, &cx.requester).await;
    if data.is_none() {
        cx.reply_to("Download error.").send().await?;
        return Err(HandlerError::from_str("Download error."));
    }
    let data = data.unwrap();

    let for_accept = captures.get(1).unwrap().as_str() == "A";
    let default_file_name = String::from("file.gif");
    let file_name = animation.file_name.as_ref().unwrap_or(&default_file_name);
    if let Ok(_) = pic_repo.get_pic(file_name.to_string(), for_accept).await {
        cx.reply_to("Pic with this name and mark already exists.")
            .send()
            .await?;
    } else {
        if let Err(_) = pic_repo
            .save_pic(Pic::new(file_name.to_string(), for_accept, data))
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
