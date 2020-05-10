use std::io::Error as IoError;

pub type Result<T> = std::result::Result<T, GeneretoError>;


#[derive(Debug)]
pub enum GeneretoError{
    IoError(IoError)
}
impl std::error::Error for GeneretoError{}

impl From<IoError> for GeneretoError {
    fn from(io: IoError) -> Self {
        Self::IoError(io)
    }
}