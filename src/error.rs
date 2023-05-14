use serde_yaml::Error as SerdeYamlError;
use std::io::Error as IoError;

pub type Result<T> = std::result::Result<T, GeneretoError>;

#[derive(anyhow, Debug)]
pub enum GeneretoError {
    IoError(IoError),
    SerdeYaml(SerdeYamlError),
}

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
