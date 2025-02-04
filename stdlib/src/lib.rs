#![no_std]

extern crate alloc;

use alloc::{boxed::Box, sync::Arc, vec, vec::Vec};

use assembly::{
    mast::MastForest,
    utils::{sync::LazyLock, Deserializable},
    Library,
};
use processor::{EventHandler, HostLibrary};
use vm_core::AdviceProvider;

pub mod dsa;

mod event_handlers;
pub use event_handlers::{
    DefaultFalconSigner, FalconDivEventHandler, FalconSigToStackEventHandler,
};

// STANDARD LIBRARY
// ================================================================================================

/// The compiled representation of the Miden standard library.
#[derive(Clone)]
pub struct StdLibrary<F = DefaultFalconSigner> {
    lib: Library,
    falcon_sig_event_handler: F,
}

impl AsRef<Library> for StdLibrary {
    fn as_ref(&self) -> &Library {
        &self.lib
    }
}

impl From<StdLibrary> for Library {
    fn from(stdlib: StdLibrary) -> Self {
        stdlib.lib
    }
}

impl StdLibrary {
    /// Serialized representation of the Miden standard library.
    pub const SERIALIZED: &'static [u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/assets/std.masl"));

    /// Creates a new instance of the Miden standard library.
    fn new(lib: Library) -> Self {
        Self {
            lib,
            falcon_sig_event_handler: DefaultFalconSigner,
        }
    }

    /// Allows to customize the event handler for the [`EVENT_FALCON_SIG_TO_STACK`] event.
    pub fn with_falcon_sig_handler<F>(self, falcon_sig_event_handler: F) -> StdLibrary<F> {
        StdLibrary { lib: self.lib, falcon_sig_event_handler }
    }

    /// Returns a reference to the [MastForest] underlying the Miden standard library.
    pub fn mast_forest(&self) -> &Arc<MastForest> {
        self.lib.mast_forest()
    }
}

impl Default for StdLibrary {
    fn default() -> Self {
        static STDLIB: LazyLock<StdLibrary> = LazyLock::new(|| {
            let contents =
                Library::read_from_bytes(StdLibrary::SERIALIZED).expect("failed to read std masl!");
            StdLibrary::new(contents)
        });
        STDLIB.clone()
    }
}

impl HostLibrary for StdLibrary {
    fn get_event_handlers<A>(&self) -> Vec<Box<dyn EventHandler<A>>>
    where
        A: AdviceProvider + 'static,
    {
        vec![
            Box::new(FalconSigToStackEventHandler::new(Box::new(
                self.falcon_sig_event_handler.clone(),
            ))),
            Box::new(FalconDivEventHandler),
        ]
    }

    fn get_mast_forest(&self) -> Arc<MastForest> {
        self.mast_forest().clone()
    }
}

// EVENTS
// ================================================================================================

// Randomly generated constant values for the standard library's events. All values were sampled
// between 0 and 2^32.
pub use constants::*;

#[rustfmt::skip]
mod constants {
    /// Reads two words from the stack and pushes values onto the advice stack which are required
    /// for verification of Falcon DSA in Miden VM.
    ///
    /// Inputs:
    ///   Operand stack: [PK, MSG, ...]
    ///   Advice stack: [...]
    ///
    /// Outputs:
    ///   Operand stack: [PK, MSG, ...]
    ///   Advice stack: [SIG_DATA]
    ///
    /// Where PK is the public key corresponding to the signing key, MSG is the message, SIG_DATA
    /// is the signature data.
    pub const EVENT_FALCON_SIG_TO_STACK: u32 = 3419226139;

    /// Pushes the result of divison (both the quotient and the remainder) of a [u64] by the Falcon
    /// prime (M = 12289) onto the advice stack.
    ///
    /// Inputs:
    ///   Operand stack: [a1, a0, ...]
    ///   Advice stack: [...]
    ///
    /// Outputs:
    ///   Operand stack: [a1, a0, ...]
    ///   Advice stack: [q0, q1, r, ...]
    ///
    /// Where (a0, a1) are the 32-bit limbs of the dividend (with a0 representing the 32 least
    /// significant bits and a1 representing the 32 most significant bits).
    /// Similarly, (q0, q1) represent the quotient and r the remainder.
    pub const EVENT_FALCON_DIV: u32          = 3419226155;
}

#[cfg(test)]
mod tests {
    use assembly::LibraryPath;

    use super::*;

    #[test]
    fn test_compile() {
        let path = "std::math::u64::overflowing_add".parse::<LibraryPath>().unwrap();
        let stdlib = StdLibrary::default();
        let exists = stdlib.lib.module_infos().any(|module| {
            module
                .procedures()
                .any(|(_, proc)| module.path().clone().append(&proc.name).unwrap() == path)
        });

        assert!(exists);
    }
}
