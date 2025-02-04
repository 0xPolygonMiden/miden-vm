#![no_std]

extern crate alloc;

use alloc::{boxed::Box, sync::Arc, vec, vec::Vec};

use assembly::{
    mast::MastForest,
    utils::{sync::LazyLock, Deserializable},
    Library,
};
use event_handlers::{
    Ext2iNTTEventHandler, FalconDivEventHandler, FalconSigToStackEventHandler,
    PushSmtPeekEventHandler, U64DivEventHandler,
};
use processor::{EventHandler, HostLibrary};
use vm_core::AdviceProvider;

pub mod dsa;

mod errors;
pub use errors::Ext2InttError;

mod event_handlers;
pub use event_handlers::DefaultFalconSigner;

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
            Box::new(U64DivEventHandler),
            Box::new(Ext2iNTTEventHandler),
            Box::new(PushSmtPeekEventHandler),
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
    /// Given evaluations of a polynomial over some specified domain, interpolates the evaluations
    ///  into a polynomial in coefficient form and pushes the result into the advice stack.
    ///
    /// The interpolation is performed using the iNTT algorithm. The evaluations are expected to be
    /// in the quadratic extension.
    ///
    /// Inputs:
    ///   Operand stack: [output_size, input_size, input_start_ptr, ...]
    ///   Advice stack: [...]
    ///
    /// Outputs:
    ///   Operand stack: [output_size, input_size, input_start_ptr, ...]
    ///   Advice stack: [coefficients...]
    ///
    /// - `input_size` is the number of evaluations (each evaluation is 2 base field elements).
    ///   Must be a power of 2 and greater 1.
    /// - `output_size` is the number of coefficients in the interpolated polynomial (each
    ///   coefficient is 2 base field elements). Must be smaller than or equal to the number of
    ///   input evaluations.
    /// - `input_start_ptr` is the memory address of the first evaluation.
    /// - `coefficients` are the coefficients of the interpolated polynomial such that lowest
    ///   degree coefficients are located at the top of the advice stack.
    pub const EVENT_EXT2_INTT: u32           = 1347499010;

    /// Reads two words from the stack and pushes values onto the advice stack which are required
    /// for verification of Falcon DSA in Miden VM.
    ///
    /// Inputs:
    ///   Operand stack: [PK, MSG, ...]
    ///   Advice stack: [...]
    ///
    /// Outputs:
    ///   Operand stack: [PK, MSG, ...]
    ///   Advice stack: \[SIG_DATA\]
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

    /// Pushes onto the advice stack the value associated with the specified key in a Sparse
    /// Merkle Tree defined by the specified root.
    ///
    /// If no value was previously associated with the specified key, [ZERO; 4] is pushed onto
    /// the advice stack.
    ///
    /// Inputs:
    ///   Operand stack: [KEY, ROOT, ...]
    ///   Advice stack: [...]
    ///
    /// Outputs:
    ///   Operand stack: [KEY, ROOT, ...]
    ///   Advice stack: [VALUE, ...]
    pub const EVENT_SMT_PEEK: u32            = 1889584556;

    /// Pushes the result of [u64] division (both the quotient and the remainder) onto the advice
    /// stack.
    ///
    /// Inputs:
    ///   Operand stack: [b1, b0, a1, a0, ...]
    ///   Advice stack: [...]
    ///
    /// Outputs:
    ///   Operand stack: [b1, b0, a1, a0, ...]
    ///   Advice stack: [q0, q1, r0, r1, ...]
    ///
    /// Where (a0, a1) and (b0, b1) are the 32-bit limbs of the dividend and the divisor
    /// respectively (with a0 representing the 32 lest significant bits and a1 representing the
    /// 32 most significant bits). Similarly, (q0, q1) and (r0, r1) represent the quotient and
    /// the remainder respectively.
    ///
    /// # Errors
    /// Returns an error if the divisor is ZERO.
    pub const EVENT_U64_DIV: u32             = 678156251;
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
