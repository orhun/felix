use std::path::PathBuf;

#[derive(Debug)]
pub enum FxError {
    Io(String),
    Dirs(String),
    GetItem,
    OpenItem,
    OpenNewWindow(String),
    Yaml(String),
    WalkDir(String),
    Encode,
    Syntect(String),
    PutItem(PathBuf),
    RemoveItem(PathBuf),
    TooSmallWindowSize,
    Log(String),
    Panic,
    Extract(String),
}

impl std::error::Error for FxError {}

impl std::fmt::Display for FxError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let printable = match self {
            FxError::Io(s) => s.to_owned(),
            FxError::Dirs(s) => s.to_owned(),
            FxError::GetItem => "Error: Cannot get item info".to_owned(),
            FxError::OpenItem => "Error: Cannot open item".to_owned(),
            FxError::OpenNewWindow(s) => s.to_owned(),
            FxError::Yaml(s) => s.to_owned(),
            FxError::WalkDir(s) => s.to_owned(),
            FxError::Encode => "Error: Incorrect encoding".to_owned(),
            FxError::Syntect(s) => s.to_owned(),
            FxError::PutItem(s) => format!("Error: Cannot copy -> {:?}", s),
            FxError::RemoveItem(s) => format!("Error: Cannot remove -> {:?}", s),
            FxError::TooSmallWindowSize => "Error: Too small window size".to_owned(),
            FxError::Log(s) => s.to_owned(),
            FxError::Panic => "Error: felix panicked".to_owned(),
            FxError::Extract(s) => s.to_owned(),
        };
        write!(f, "{}", printable)
    }
}

impl From<std::io::Error> for FxError {
    fn from(err: std::io::Error) -> Self {
        FxError::Io(err.to_string())
    }
}
impl From<serde_yaml::Error> for FxError {
    fn from(err: serde_yaml::Error) -> Self {
        FxError::Yaml(err.to_string())
    }
}

impl From<syntect::Error> for FxError {
    fn from(err: syntect::Error) -> Self {
        FxError::Syntect(err.to_string())
    }
}

impl From<walkdir::Error> for FxError {
    fn from(err: walkdir::Error) -> Self {
        FxError::WalkDir(err.to_string())
    }
}

impl From<log::SetLoggerError> for FxError {
    fn from(err: log::SetLoggerError) -> Self {
        FxError::Log(err.to_string())
    }
}

impl From<std::string::FromUtf8Error> for FxError {
    fn from(_err: std::string::FromUtf8Error) -> Self {
        FxError::Encode
    }
}

impl From<zip::result::ZipError> for FxError {
    fn from(err: zip::result::ZipError) -> Self {
        FxError::Extract(err.to_string())
    }
}
