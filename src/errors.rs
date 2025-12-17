use colored::Colorize;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct SyntaxErrorDescription {
    snippet: String,
    error_index: u32,
    probable_cause: String,
}

impl std::fmt::Display for SyntaxErrorDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.snippet)?;
        for _ in 0..self.error_index {
            write!(f, " ")?;
        }
        write!(f, "^")?;
        writeln!(f, "Probable cause: {}", self.probable_cause.red())
    }
}

#[derive(Debug, Clone, Error)]
pub enum EezyError {
    #[error("Invalid path to source {0} was provided.")]
    InvalidSourcePath(String),
    #[error("Failed to perform file IO operation {0}")]
    FileIOError(String),
    #[error("Syntax error at line number {0}.\n{1}")]
    SyntaxError(String, SyntaxErrorDescription),
}

impl From<std::io::Error> for EezyError {
    fn from(value: std::io::Error) -> Self {
        Self::FileIOError(value.to_string())
    }
}
