use {
    std::{
        fmt,
        io,
        process::Command,
        string::FromUtf8Error,
    },
};

#[derive(Debug, Clone)]
pub struct TermuxClipboardError {
    pub message: String,
}

impl From<String> for TermuxClipboardError {
    fn from(message: String) -> Self {
        Self { message }
    }
}

impl fmt::Display for TermuxClipboardError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "termux clipboard error: {}", self.message)
    }
}

impl From<FromUtf8Error> for TermuxClipboardError {
    fn from(_: FromUtf8Error) -> Self {
        TermuxClipboardError::from("error interpreting as UTF8".to_string())
    }
}
impl From<io::Error> for TermuxClipboardError {
    fn from(e: io::Error) -> Self {
        TermuxClipboardError::from(e.to_string())
    }
}

/// query the termux API to get the content of the
/// Android clipboard if it can be converted to a
/// string.
///
/// Won't work if the Termux API isn't installed
pub fn get_string() -> Result<String, TermuxClipboardError> {
    Ok(String::from_utf8(
        Command::new("termux-clipboard-get")
            .output()?
            .stdout
    )?)
}

/// fill the Android clipboard
///
/// Won't work if the Termux API isn't installed
pub fn set_string<S: AsRef<str>>(s: S) -> Result<(), TermuxClipboardError> {
    let status = Command::new("termux-clipboard-set")
        .arg(s.as_ref())
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(TermuxClipboardError::from(format!(
            "command termux-clipboard-set failed with status {:?}",
            status.code(),
        )))
    }
}
