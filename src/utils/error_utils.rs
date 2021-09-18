use teloxide::RequestError;

#[derive(Debug, PartialEq)]
pub struct HandlerError {
    pub message: String,
}

impl HandlerError {
    pub fn new(data: String) -> Self {
        HandlerError {
            message: data.into(),
        }
    }

    pub fn from_str(data: &str) -> Self {
        HandlerError {
            message: String::from(data).into(),
        }
    }
}

impl From<RequestError> for HandlerError {
    fn from(e: RequestError) -> Self {
        HandlerError::new(format!("Teloxide request error: {:?}", e).to_string())
    }
}

impl From<sqlx::Error> for HandlerError {
    fn from(e: sqlx::Error) -> Self {
        HandlerError::new(format!("Sqlx error: {:?}", e).to_string())
    }
}

impl<T> From<Option<T>> for HandlerError {
    fn from(_: Option<T>) -> Self {
        HandlerError::new(String::from("Option is None"))
    }
}

impl std::fmt::Display for HandlerError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.message)
    }
}
