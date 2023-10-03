use super::{AdviceExtractor, AdviceFunction, AdviceInjector, DebugOptions};
use crate::{crypto::merkle::MerklePath, Felt, Word};
use core::fmt;

// HOST FUNCTION
// ================================================================================================

/// HostFunction that can be executed by the VM against the host. Executing a host function does
/// not affect the state of the main VM components. However, host functions may modify the host.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum HostFunction {
    /// Prints out information about the state of the VM based on the specified options. This
    /// decorator is executed only in debug mode.
    Debug(DebugOptions),
    /// Injects new data into the advice provider, as specified by the injector.
    AdviceFunction(AdviceFunction),
}

impl HostFunction {
    /// Constructs a new [AdviceInjector] [HostFunction].
    pub fn new_advice_injector(injector: AdviceInjector) -> HostFunction {
        HostFunction::AdviceFunction(AdviceFunction::Injector(injector))
    }

    /// Constructs a new [AdviceExtractor] [HostFunction].
    pub fn new_advice_extractor(extractor: AdviceExtractor) -> HostFunction {
        HostFunction::AdviceFunction(AdviceFunction::Extractor(extractor))
    }
}

impl fmt::Display for HostFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AdviceFunction(advice_function) => write!(f, "{advice_function}"),
            Self::Debug(options) => write!(f, "host::debug({options})"),
        }
    }
}

// HOST RESULT
// ================================================================================================

/// Result returned by the host upon successful execution of a [HostFunction].
pub enum HostResult {
    MerklePath(MerklePath),
    DoubleWord([Word; 2]),
    Word(Word),
    Element(Felt),
    Unit,
}
