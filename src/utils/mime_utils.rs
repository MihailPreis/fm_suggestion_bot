use teloxide::types::Document;

pub fn is_image(doc: &Document) -> bool {
    doc.mime_type
        .as_ref()
        .and_then(|mime| {
            Some(vec!["image/jpeg", "image/png", "image/webp"].contains(&&*mime.to_string()))
        })
        .unwrap_or(false)
}

pub fn is_animate(doc: &Document) -> bool {
    doc.mime_type
        .as_ref()
        .and_then(|mime| Some(mime.to_string() == "image/gif"))
        .unwrap_or(false)
}

pub fn is_video(doc: &Document) -> bool {
    doc.mime_type
        .as_ref()
        .and_then(|mime| Some(vec!["video/quicktime", "video/mp4"].contains(&&*mime.to_string())))
        .unwrap_or(false)
}
