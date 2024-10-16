use dioxus::events::EvalError;
use std::fmt;
use std::num::ParseIntError;
use tgl_rando_core::tgl_error::TGLError;

pub struct GuiError {
    pub message: String,
}

pub fn gui_error(message: &str) -> GuiError {
    GuiError {
        message: message.into(),
    }
}
impl fmt::Debug for GuiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error {{message: {} }}", self.message)
    }
}

impl From<Box<dyn std::error::Error>> for GuiError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        GuiError {
            message: error.to_string(),
        }
    }
}

impl From<ParseIntError> for GuiError {
    fn from(error: ParseIntError) -> Self {
        GuiError {
            message: error.to_string(),
        }
    }
}

impl From<EvalError> for GuiError {
    fn from(_: EvalError) -> Self {
        gui_error("Error evaluating JS in Web Gui")
    }
}

impl From<TGLError> for GuiError {
    fn from(error: TGLError) -> Self {
        GuiError {
            message: error.message,
        }
    }
}

impl From<std::io::Error> for GuiError {
    fn from(error: std::io::Error) -> Self {
        GuiError {
            message: error.to_string(),
        }
    }
}

impl From<&str> for GuiError {
    fn from(str: &str) -> Self {
        GuiError {
            message: str.to_string(),
        }
    }
}

impl From<String> for GuiError {
    fn from(str: String) -> Self {
        GuiError { message: str }
    }
}
