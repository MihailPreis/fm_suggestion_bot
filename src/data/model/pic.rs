pub struct Pic {
    pub file_name: String,
    pub for_accept: bool,
    pub data: Vec<u8>,
}

impl Pic {
    pub fn new(file_name: String, for_accept: bool, data: Vec<u8>) -> Self {
        Self {
            file_name,
            for_accept,
            data,
        }
    }
}
