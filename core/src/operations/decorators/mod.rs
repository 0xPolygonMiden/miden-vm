mod advice;
pub use advice::AdviceInjector;
use core::fmt;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Decorator {
    /// Injects zero or more values at the head of the advice tape as specified by the injector.
    /// This operation affects only the advice tape, but has no effect on other VM components
    /// (e.g., stack, memory), and does not advance VM clock.
    Advice(AdviceInjector),
}

impl fmt::Display for Decorator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Advice(injector) => write!(f, "advice({})", injector),
        }
    }
}
