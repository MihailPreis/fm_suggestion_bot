pub struct UserStats {
    pub user_id: i64,
    pub offered_count: i64,
    pub accepted_count: i64,
    pub declined_count: i64,
}

impl UserStats {
    pub fn new(user_id: i64, offered_count: i64, accepted_count: i64, declined_count: i64) -> Self {
        Self {
            user_id,
            offered_count,
            accepted_count,
            declined_count,
        }
    }

    pub fn empty(user_id: i64) -> Self {
        Self::new(user_id, 0, 0, 0)
    }
}
