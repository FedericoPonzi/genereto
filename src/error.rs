use serde::export::Formatter;
use serde_yaml::Error as SerdeYamlError;
use std::fmt::Display;
use std::io::Error as IoError;

pub type Result<T> = std::result::Result<T, GeneretoError>;

#[derive(Debug)]
pub enum GeneretoError {
    IoError(IoError),
    SerdeYaml(SerdeYamlError),
}

impl std::error::Error for GeneretoError {}

impl From<IoError> for GeneretoError {
    fn from(err: IoError) -> Self {
        Self::IoError(err)
    }
}
impl From<SerdeYamlError> for GeneretoError {
    fn from(err: SerdeYamlError) -> Self {
        Self::SerdeYaml(err)
    }
}

impl Display for GeneretoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self)
    }
}
