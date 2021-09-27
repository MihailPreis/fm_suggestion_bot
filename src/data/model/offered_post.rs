pub struct OfferedPost {
    pub chat_id: i64,
    pub message_id: i32,
    pub admin_chat_id: i64,
    pub admin_chat_message_id: i32,
    pub admin_chat_additional_message_id: Option<i32>,
}

impl OfferedPost {
    pub fn new(
        chat_id: i64,
        message_id: i32,
        admin_chat_id: i64,
        admin_chat_message_id: i32,
        admin_chat_additional_message_id: Option<i32>,
    ) -> Self {
        OfferedPost {
            chat_id,
            message_id,
            admin_chat_id,
            admin_chat_message_id,
            admin_chat_additional_message_id,
        }
    }
}
