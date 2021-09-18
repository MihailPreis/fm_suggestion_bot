pub struct CachedPic {
    pub image_name: String,
    pub image_file_id: String,
}

impl CachedPic {
    pub fn new(image_name: String, image_file_id: String) -> Self {
        CachedPic {
            image_name,
            image_file_id,
        }
    }
}
