use bytes::BufMut;
use futures_util::TryStreamExt;
use teloxide::net::Download;
use teloxide::prelude::{Request, Requester};
use teloxide::types::Document;
use teloxide::Bot;

pub async fn download_vec(doc: &Document, bot: &Bot) -> Option<Vec<u8>> {
    let file = bot.get_file(doc.file_id.to_string()).send().await.ok()?;
    let stream = bot.download_file_stream(&file.file_path);
    return stream
        .try_fold(Vec::new(), |mut vec, data| {
            vec.put(data);
            async move { Ok(vec) }
        })
        .await
        .ok();
}
