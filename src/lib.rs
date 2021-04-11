use std::io;
use std::fmt;
use std::path;

#[derive(Debug)]
pub enum RenameError {
    NotFound(io::Error),
    PermissionDenied(io::Error),
    UnexpectedError(io::Error),
}

impl fmt::Display for RenameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            RenameError::NotFound(ref e)         => write!(f, "Unable to rename(NotFound): {}", e),
            RenameError::PermissionDenied(ref e) => write!(f, "Unable to rename(PermissionDenied): {}", e),
            RenameError::UnexpectedError(ref e) => write!(f, "Unable to rename: {}", e),
        }
    }
}

pub fn rename<S,D>(src: S, dst: D) -> Result<(), RenameError>
where S: AsRef<path::Path>,
      D: AsRef<path::Path>
{
    let r: Result<(), io::Error> = std::fs::rename(src, dst);
    match r {
        Ok(_) => Ok(()),
        Err(e) => {
            match e.kind(){
                io::ErrorKind::NotFound         => Err(RenameError::NotFound(e)),
                io::ErrorKind::PermissionDenied => Err(RenameError::PermissionDenied(e)),
                _                               => Err(RenameError::UnexpectedError(e)),
            }
        },
    }
}
