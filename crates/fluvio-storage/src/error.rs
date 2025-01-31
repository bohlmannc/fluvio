use std::io::Error as IoError;

use fluvio_future::fs::BoundedFileSinkError;
use fluvio_future::zero_copy::SendFileError;

use crate::util::OffsetError;
use crate::validator::LogValidationError;

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error(transparent)]
    Io(#[from] IoError),
    #[error("Offset error")]
    Offset(#[from] OffsetError),
    #[error("Log validation error")]
    LogValidation(#[from] LogValidationError),
    #[error("Zero-copy send file error")]
    SendFile(#[from] SendFileError),
    #[error("Batch exceeded maximum bytes: {0}")]
    BatchTooBig(usize),
}

impl From<BoundedFileSinkError> for StorageError {
    fn from(error: BoundedFileSinkError) -> Self {
        match error {
            BoundedFileSinkError::IoError(err) => StorageError::Io(err),
            BoundedFileSinkError::MaxLenReached => panic!("no auto conversion for file sink error"),
        }
    }
}
