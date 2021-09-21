use bytes::BufMut;
use futures_util::TryStreamExt;
use teloxide::net::Download;
use teloxide::prelude::{Request, Requester};
use teloxide::types::{Animation, Document};
use teloxide::Bot;

pub async fn download_doc_vec(doc: &Document, bot: &Bot) -> Option<Vec<u8>> {
    _download(doc.file_id.to_string(), bot).await
}

pub async fn download_animate_vec(doc: &Animation, bot: &Bot) -> Option<Vec<u8>> {
    _download(doc.file_id.to_string(), bot).await
}

async fn _download(file_id: String, bot: &Bot) -> Option<Vec<u8>> {
    let file = bot.get_file(file_id).send().await.ok()?;
    let stream = bot.download_file_stream(&file.file_path);
    return stream
        .try_fold(Vec::new(), |mut vec, data| {
            vec.put(data);
            async move { Ok(vec) }
        })
        .await
        .ok();
}
