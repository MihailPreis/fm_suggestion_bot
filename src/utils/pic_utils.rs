use crate::data::repo::cached_pic_repo::CachedPicRepo;
use crate::data::repo::pic_repo::PicRepo;

pub enum GetPicResult {
    Raw(String, Vec<u8>),
    FileId(String),
}

pub async fn get_pic(
    is_accept: bool,
    cached_pic_repo: &CachedPicRepo,
    pic_repo: &PicRepo,
) -> Option<GetPicResult> {
    if let Ok(pic) = pic_repo.get_random_pic(is_accept).await {
        if let Ok(cached) = cached_pic_repo
            .get_cached_pic(pic.file_name.to_string())
            .await
        {
            Some(GetPicResult::FileId(cached.image_file_id))
        } else {
            Some(GetPicResult::Raw(pic.file_name.to_string(), pic.data))
        }
    } else {
        None
    }
}
