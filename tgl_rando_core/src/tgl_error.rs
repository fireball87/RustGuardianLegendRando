use std::fmt;
use std::num::ParseIntError;

pub struct TGLError {
    pub message: String,
}

pub fn tgl_error(message: &str) -> TGLError {
    TGLError {
        message: message.into(),
    }
}
impl fmt::Debug for TGLError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error {{message: {} }}", self.message)
    }
}

impl From<Box<dyn std::error::Error>> for TGLError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        TGLError {
            message: error.to_string(),
        }
    }
}

impl From<ParseIntError> for TGLError {
    fn from(error: ParseIntError) -> Self {
        TGLError {
            message: error.to_string(),
        }
    }
}

impl From<hex::FromHexError> for TGLError {
    fn from(error: hex::FromHexError) -> Self {
        TGLError {
            message: error.to_string(),
        }
    }
}

impl From<std::io::Error> for TGLError {
    fn from(error: std::io::Error) -> Self {
        TGLError {
            message: error.to_string(),
        }
    }
}

impl From<&str> for TGLError {
    fn from(str: &str) -> Self {
        TGLError {
            message: str.to_string(),
        }
    }
}

impl From<String> for TGLError {
    fn from(str: String) -> Self {
        TGLError { message: str }
    }
}
