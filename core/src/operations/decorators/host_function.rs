use core::fmt;

use super::AdviceInjector;
use super::DebugOptions;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum HostFunction {
    /// Prints out information about the state of the VM based on the specified options. This
    /// decorator is executed only in debug mode.
    Debug(DebugOptions),
    /// Injects new data into the advice provider, as specified by the injector.
    AdviceInjector(AdviceInjector),
    /// Advances the clock of the host by one cycle.
    AdvanceClock,
}

impl fmt::Display for HostFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AdvanceClock => write!(f, "host::advance_clock"),
            Self::AdviceInjector(injector) => write!(f, "host::advice_injector({injector})"),
            Self::Debug(options) => write!(f, "host::debug({options})"),
        }
    }
}
