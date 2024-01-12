use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

pub type Result<T> = std::result::Result<T, EvaluateError>;

#[derive(Debug)]
pub enum EvaluateError {
    ThreadError { message: String },
    IoError { error: std::io::Error },
    FormatError { message: String },
}
impl Error for EvaluateError {}

impl Display for EvaluateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ThreadError { message } => {
                write!(f, "Thread error: {}", message)
            }
            Self::IoError { error } => {
                write!(f, "IO error: {}", error)
            }
            Self::FormatError { message } => {
                write!(f, "Format error: {}", message)
            }
        }
    }
}

impl From<std::io::Error> for EvaluateError {
    fn from(value: std::io::Error) -> Self {
        EvaluateError::IoError { error: value }
    }
}
