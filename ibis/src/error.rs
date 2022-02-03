use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum IbisError {}

impl fmt::Display for IbisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IbisError")?;
        todo!();
    }
}

impl Error for IbisError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        todo!()
    }
}
