use log::error;

pub trait FatalValueMapper<T> {
    fn map_value_or_exit(self, message: String) -> T;
}

impl<T, E> FatalValueMapper<T> for Result<T, E> {
    fn map_value_or_exit(self, message: String) -> T {
        self.unwrap_or_else(|_e| {
            error!("{}", message);
            std::process::exit(1)
        })
    }
}
