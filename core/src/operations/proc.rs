use core::fmt;

/// Stores procedure information
#[derive(Clone, Debug, PartialEq)]
pub struct ProcInfo {
    name: String,
    locals: usize,
}

impl fmt::Display for ProcInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "proc.{}.{}", self.name, self.locals)?;
        Ok(())
    }
}
