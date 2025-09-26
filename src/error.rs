pub trait ConsoleErrorUnwrap {
    type Item;
    fn cunwrap(self, error_msg: &str) -> Self::Item;
}
impl<T, ANY: std::fmt::Debug> ConsoleErrorUnwrap for Result<T, ANY> {
    type Item = T;
    fn cunwrap(self, error_msg: &str) -> Self::Item {
        if self.is_err() {
            error_out(error_msg);
        }
        self.unwrap()
    }
}
impl<T> ConsoleErrorUnwrap for Option<T> {
    type Item = T;
    fn cunwrap(self, error_msg: &str) -> Self::Item {
        if self.is_none() {
            error_out(error_msg);
        }
        self.unwrap()
    }
}

pub fn error_out(message: &str) {
    web_sys::console::error_1(&message.into());
}