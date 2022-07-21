use std::io::Error;

pub type Result<T> = std::result::Result<T, String>;

impl From<Error> for String {
    fn from(_: std::io::Error) -> Self {
        todo!()
    }
}
