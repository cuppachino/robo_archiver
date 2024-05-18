use std::fmt::Formatter;
use derive_more::From;

#[derive(Debug, From)]
pub enum ArchiveError {
    Io(std::io::Error),
    Csv(csv::Error),
    Unimplemented,
    UnparseableFileName(String),
}

impl std::fmt::Display for ArchiveError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            ArchiveError::Io(err) => write!(f, "I/O error: {}", err),
            ArchiveError::Unimplemented => write!(f, "Unimplemented"),
            ArchiveError::UnparseableFileName(file) => write!(f, "Unparseable file name: {}", file),
            ArchiveError::Csv(err) => write!(f, "CSV error: {}", err),
        }
    }
}
