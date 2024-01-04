use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

pub type Result<T> = std::result::Result<T, EvaluateError>;

#[derive(Debug)]
pub enum EvaluateError {
    ThreadError { message: String },
}

impl Display for EvaluateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EvaluateError::ThreadError { message } => {
                write!(f, "Thread error: {}", message)
            }
        }
    }
}

impl Error for EvaluateError {}
