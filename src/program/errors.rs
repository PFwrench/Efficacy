#[derive(Debug)]
pub enum EfficacyError {
    ConfigError(config::ConfigError),
    IOError(std::io::Error),
    SerdeJsonError(serde_json::Error),
    MismatchedIdError,
    NonexistentCategoryError,
    Other,
}

impl std::error::Error for EfficacyError {}

impl std::fmt::Display for EfficacyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "An error occurred:")
    }
}

impl std::convert::From<config::ConfigError> for EfficacyError {
    fn from(config_error: config::ConfigError) -> EfficacyError {
        EfficacyError::ConfigError(config_error)
    }
}

impl std::convert::From<std::io::Error> for EfficacyError {
    fn from(io_error: std::io::Error) -> EfficacyError {
        EfficacyError::IOError(io_error)
    }
}

impl std::convert::From<serde_json::Error> for EfficacyError {
    fn from(serde_json_error: serde_json::Error) -> EfficacyError {
        EfficacyError::SerdeJsonError(serde_json_error)
    }
}

// impl std::convert::From<std::option::NoneError> for EfficacyError {
//     fn from(none_error: std::option::NoneError) -> EfficacyError {
//         EfficacyError::
//     }
// }
