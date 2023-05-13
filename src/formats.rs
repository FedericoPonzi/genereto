use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;

pub struct MarkdownFile<'a>(pub(crate) &'a str);

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub template: String,
}

impl Config {
    pub fn load_from_path<P: AsRef<Path>>(config_path: P) -> Result<Self> {
        serde_yaml::from_reader(&File::open(config_path.as_ref())?).map_err(Into::into)
    }
}
