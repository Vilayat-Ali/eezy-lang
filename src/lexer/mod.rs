pub mod token;

use crate::errors::EezyError;
use std::path::PathBuf;

pub struct Lexer {
    pub source_path: PathBuf,
}

impl Lexer {
    pub fn new(path_to_source: Vec<PathBuf>) -> Result<Self, EezyError> {
        let source_path = path_to_source.iter().collect::<PathBuf>();

        if !source_path.exists() {
            return Err(EezyError::InvalidSourcePath(
                source_path.display().to_string(),
            ));
        }

        Ok(Self { source_path })
    }
}
