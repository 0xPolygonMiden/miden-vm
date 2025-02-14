#![no_std]

extern crate alloc;

use alloc::{boxed::Box, sync::Arc, vec, vec::Vec};
use core::error::Error;

use assembly::{
    mast::MastForest,
    utils::{sync::LazyLock, Deserializable},
    Library,
};
use processor::{
    AdviceProvider, AdviceProviderError, EventHandler, HostLibrary, ProcessState, Word,
};

pub mod dsa;

// STANDARD LIBRARY
// ================================================================================================

/// Serialized representation of the Miden standard library.
pub const SERIALIZED: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/assets/std.masl"));
static STDLIB: LazyLock<Library> =
    LazyLock::new(|| Library::read_from_bytes(SERIALIZED).expect("failed to read std masl!"));

/// The compiled representation of the Miden standard library.
#[derive(Clone)]
pub struct StdLibrary {
    lib: Library,
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
    /// Creates a new instance of the Miden standard library.
    ///
    /// This function is different from [Default::default] in that it allows the caller to specify a
    /// type for the Falcon signer, and the `Inputs` required to build it.
    pub fn new() -> Self {
        Self { lib: STDLIB.clone() }
    }

    /// Returns a reference to the [MastForest] underlying the Miden standard library.
    pub fn mast_forest(&self) -> &Arc<MastForest> {
        self.lib.mast_forest()
    }

    /// Creates a new instance of the Miden standard library.
    fn new_with_lib(lib: Library) -> Self {
        Self { lib }
    }
}

impl Default for StdLibrary {
    fn default() -> Self {
        StdLibrary::new_with_lib(STDLIB.clone())
    }
}

impl HostLibrary<FalconSignerClosure> for StdLibrary {
    fn get_event_handlers(&self, signer: FalconSignerClosure) -> Vec<Box<dyn EventHandler>> {
        vec![Box::new(FalconSigToStackEventHandler::new(signer))]
    }

    fn get_mast_forest(&self) -> Arc<MastForest> {
        self.mast_forest().clone()
    }
}

// EVENTS
// ================================================================================================

pub const EVENT_FALCON_SIG_TO_STACK: u32 = 3419226139;

// EVENT HANDLERS
// ==============================================================================================

/// Takes in `pub_key: Word`, `msg: Word` and `advice_provider: A` and writes the Falcon signature onto the advice stack.
pub type FalconSignerClosure =
    Box<dyn FnMut(Word, Word) -> Result<(), Box<dyn Error + Send + Sync + 'static>>>;

/// An event handler which generates a Falcon signature and pushes the result onto the stack.
pub struct FalconSigToStackEventHandler {
    signer: FalconSignerClosure,
}

impl FalconSigToStackEventHandler {
    /// Creates a new instance of the Falcon signature to stack event handler, given a specified
    /// Falcon signer.
    pub fn new(signer: FalconSignerClosure) -> Self {
        Self { signer }
    }
}

// FIXME(plafer): Can't have Default implementation anymore
// impl Default for FalconSigToStackEventHandler
// {
//     fn default() -> Self {
//         Self {
//             signer: Box::new(|msg, pub_key| {
//                 let priv_key = advice_provider
//                     .get_mapped_values(&pub_key.into())
//                     .ok_or(AdviceProviderError::AdviceMapKeyNotFound(pub_key))?;

//                 dsa::falcon_sign(priv_key, msg)
//             }),
//         }
//     }
// }

impl EventHandler for FalconSigToStackEventHandler {
    fn id(&self) -> u32 {
        EVENT_FALCON_SIG_TO_STACK
    }

    fn on_event(
        &mut self,
        process: ProcessState,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let pub_key = process.get_stack_word(0);
        let msg = process.get_stack_word(1);

        // pushes the signature onto the advice stack
        (self.signer)(pub_key, msg)?;

        // Note(plafer): this is now done by the signer itself, since `on_event()` no longer has
        // an advice provider
        // for r in signature {
        //     advice_provider.push_stack(AdviceSource::Value(r))?;
        // }

        Ok(())
    }
}

// TODO(plafer): Double-check if `Send + Sync` is necessary.
// If not, remove it from miden-base's `TransactionAuthenticator`, and `BasicAuthenticator`
/// A trait for signing messages using the Falcon signature scheme.
///
/// This trait is used by [FalconSigToStackEventHandler] to sign messages using the Falcon signature
/// scheme.
///
/// It is recommended to use [dsa::falcon_sign] to implement this trait once the private key has
/// been fetched from a user-defined location.
pub trait FalconSigner<Inputs>: Send + Sync {
    fn new(args: Inputs) -> Self;

    /// Signs the message using the Falcon signature scheme, and returns the signature as a
    /// `Vec<Felt>`.
    fn sign_message<A>(
        &mut self,
        pub_key: Word,
        msg: Word,
        advice_provider: &A,
    ) -> Result<Vec<Felt>, Box<dyn Error + Send + Sync + 'static>>
    where
        A: AdviceProvider;
}

/// The default Falcon signer.
///
/// This signer reads the private key from the advice provider's map using `pub_key` as the map key,
/// and signs the message.
#[derive(Debug, Clone)]
pub struct DefaultFalconSigner;

impl<Inputs> FalconSigner<Inputs> for DefaultFalconSigner {
    fn new(_args: Inputs) -> Self {
        Self
    }

    fn sign_message<A>(
        &mut self,
        pub_key: Word,
        msg: Word,
        advice_provider: &A,
    ) -> Result<Vec<Felt>, Box<dyn Error + Send + Sync + 'static>>
    where
        A: AdviceProvider,
    {
        let priv_key = advice_provider
            .get_mapped_values(&pub_key.into())
            .ok_or(AdviceProviderError::AdviceMapKeyNotFound(pub_key))?;

        dsa::falcon_sign(priv_key, msg)
    }
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
