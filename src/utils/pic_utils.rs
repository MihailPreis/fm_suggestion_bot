use crate::data::repo::cached_pic_repo::CachedPicRepo;
use crate::utils::env_utils::try_get_env;
use lazy_static::lazy_static;
use rand::seq::IteratorRandom;
use std::fs::{metadata, read_dir, File};
use std::io::Read;
use std::path::Path;

static ACCEPT_FILES_KEY: &str = "ACCEPT_FILES";
static DECLINE_FILES_KEY: &str = "DECLINE_FILES";

lazy_static! {
    static ref ACCEPT_FILES: Option<String> = try_get_env(ACCEPT_FILES_KEY);
    static ref DECLINE_FILES: Option<String> = try_get_env(DECLINE_FILES_KEY);
}

pub enum GetPicResult {
    Raw(String, Vec<u8>),
    FileId(String),
}

pub async fn get_pic(is_accept: bool, cached_pic_repo: &CachedPicRepo) -> Option<GetPicResult> {
    let path: &Option<String> = if is_accept {
        &ACCEPT_FILES
    } else {
        &DECLINE_FILES
    };
    let path = path.as_ref()?;
    let meta = metadata(&path).ok()?;
    if meta.is_dir() {
        let paths = read_dir(&path).ok()?;
        let path = paths
            .filter_map(|f| f.ok())
            .filter(|f| is_mp4_path(f.path().as_path()))
            .choose(&mut rand::thread_rng())
            .and_then(|f| f.path().to_str().and_then(|s| Some(s.to_string())))?;
        _get(cached_pic_repo, &&path).await
    } else if meta.is_file() {
        if !is_mp4(&path) {
            log::warn!(
                "File in {} env key not a MP4.",
                if is_accept {
                    ACCEPT_FILES_KEY
                } else {
                    DECLINE_FILES_KEY
                }
            );
            return None;
        }
        _get(cached_pic_repo, &path).await
    } else {
        log::warn!(
            "{} env key not a file or directory.",
            if is_accept {
                ACCEPT_FILES_KEY
            } else {
                DECLINE_FILES_KEY
            }
        );
        None
    }
}

async fn _get(cached_pic_repo: &CachedPicRepo, path: &&String) -> Option<GetPicResult> {
    let filename = Path::new(&path)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    if let Some(cached_pic) = cached_pic_repo
        .get_cached_pic(filename.to_string())
        .await
        .ok()
    {
        Some(GetPicResult::FileId(cached_pic.image_file_id))
    } else {
        get_file_as_vec(path.to_string()).and_then(|raw| Some(GetPicResult::Raw(filename, raw)))
    }
}

fn get_file_as_vec(path: String) -> Option<Vec<u8>> {
    let mut file = File::open(&path).ok()?;
    let metadata = metadata(&path).ok()?;
    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer).ok()?;
    Some(buffer)
}

fn is_mp4(path: &String) -> bool {
    is_mp4_path(Path::new(&path))
}

fn is_mp4_path(path: &Path) -> bool {
    match path.extension() {
        Some(path) => path.to_ascii_lowercase() == "mp4",
        None => false,
    }
}
