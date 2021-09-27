use teloxide::prelude::*;

use crate::utils::error_utils::HandlerError;

pub async fn unwrap_send_error<T>(
    value: Option<T>,
    cx: &UpdateWithCx<Bot, Message>,
    msg: &str,
) -> Result<T, HandlerError> {
    if value.is_none() {
        cx.reply_to(msg).send().await?;
        return Err(HandlerError::from_str(msg));
    }
    Ok(value.unwrap())
}
