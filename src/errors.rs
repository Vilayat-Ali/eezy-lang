use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum EezyError<'a> {
    #[error("Invalid path to source {0} was provided.")]
    InvalidSourcePath(&'a String)
}