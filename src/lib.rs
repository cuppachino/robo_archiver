use std::fmt::Formatter;

#[derive(Debug)]
pub enum ArchiveError {
    Io(std::io::Error),
    Unimplemented,
    UnparseableFileName(String),
}

impl From<std::io::Error> for ArchiveError {
    fn from(err: std::io::Error) -> Self {
        ArchiveError::Io(err)
    }
}

impl std::fmt::Display for ArchiveError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            ArchiveError::Io(err) => write!(f, "I/O error: {}", err),
            ArchiveError::Unimplemented => write!(f, "Unimplemented"),
            ArchiveError::UnparseableFileName(file) => write!(f, "Unparseable file name: {}", file),
        }
    }
}
