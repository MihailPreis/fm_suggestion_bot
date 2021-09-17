use teloxide::types::User;

pub(crate) trait ExtUser {
    fn ftm_title(&self) -> String;
}

impl ExtUser for User {
    fn ftm_title(&self) -> String {
        let _username = &self
            .username
            .as_ref()
            .and_then(|username| Some(format!("<@{}>", username)))
            .unwrap_or(String::new());
        let empty = &String::new();
        let _last_name = &self.last_name.as_ref().unwrap_or(empty);
        return [_username, &self.first_name, _last_name]
            .iter()
            .filter(|item| !item.is_empty())
            .map(|item| item.to_string())
            .collect::<Vec<String>>()
            .join(" ");
    }
}
