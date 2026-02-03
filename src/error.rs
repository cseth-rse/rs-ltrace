use std::fmt;

#[derive(Debug)]
pub enum TraceError {
    NixError(String),
    AttachFailed,
    WaitFailed,
    PtrError(String),
    InvalidPid,
}

impl fmt::Display for TraceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TraceError::NixError(e) => write!(f, "Nix error: {}", e),
            TraceError::AttachFailed => write!(f, "Failed to attach to process"),
            TraceError::WaitFailed => write!(f, "Waitpid failed"),
            TraceError::PtrError(e) => write!(f, "Ptrace error: {}", e),
            TraceError::InvalidPid => write!(f, "Invalid PID"),
        }
    }
}

impl std::error::Error for TraceError {}
