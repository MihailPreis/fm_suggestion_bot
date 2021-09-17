use teloxide::types::Message;

pub(crate) trait ExtMessage {
    fn has_caption(&self) -> bool;
}

impl ExtMessage for Message {
    fn has_caption(&self) -> bool {
        self.caption().unwrap_or("").len() > 0 || self.caption_entities().unwrap_or(&[]).len() > 0
    }
}
