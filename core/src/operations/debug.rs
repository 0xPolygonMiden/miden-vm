use core::fmt;

/// TODO: add docs
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DebugOptions {
    All,
    Stack(Option<usize>),
    Memory(Option<u64>, Option<u64>),
    Local(Option<usize>),
}

impl fmt::Display for DebugOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DebugOptions::All => write!(f, "all"),
            DebugOptions::Stack(n) => match n {
                None => write!(f, "stack"),
                Some(n) => write!(f, "stack.{}", n),
            },
            DebugOptions::Memory(n, m) => match n {
                None => write!(f, "mem"),
                Some(n) => match m {
                    None => write!(f, "mem.{}", n),
                    Some(m) => write!(f, "mem.{}.{}", n, m),
                },
            },
            DebugOptions::Local(n) => match n {
                None => write!(f, "local"),
                Some(n) => write!(f, "local.{}", n),
            },
        }
    }
}
