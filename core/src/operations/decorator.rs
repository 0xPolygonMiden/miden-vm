use super::AdviceInjector;
use crate::utils::string::String;
use core::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Decorator {
    AsmOp(String),
    Advice(AdviceInjector),
    ProcStart(String),
    ProcEnd(String),
}

impl fmt::Display for Decorator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AsmOp(op) => write!(f, "asmOp({})", op),
            Self::Advice(injector) => write!(f, "advice({})", injector),
            Self::ProcStart(proc) => write!(f, "procStart({})", proc),
            Self::ProcEnd(proc) => write!(f, "procEnd({})", proc),
        }
    }
}
