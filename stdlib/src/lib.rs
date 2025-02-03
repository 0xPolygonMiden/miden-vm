#![no_std]

extern crate alloc;

use alloc::{boxed::Box, sync::Arc, vec, vec::Vec};
use core::error::Error;

use assembly::{
    mast::MastForest,
    utils::{sync::LazyLock, Deserializable},
    Library,
};
use processor::{EventHandler, HostLibrary, ProcessState};
use vm_core::{AdviceProvider, AdviceProviderError, AdviceSource, Felt, Word};

pub mod dsa;

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
        vec![Box::new(FalconSigToStackEventHandler::new(Box::new(
            self.falcon_sig_event_handler.clone(),
        )))]
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

/// An event handler which verifies a Falcon signature and pushes the result onto the stack.
pub struct FalconSigToStackEventHandler<A> {
    signer: Box<dyn FalconSigner<A>>,
}

impl<A> FalconSigToStackEventHandler<A> {
    /// Creates a new instance of the Falcon signature to stack event handler, given a specified
    /// Falcon signer.
    pub fn new(signer: Box<dyn FalconSigner<A>>) -> Self {
        Self { signer }
    }
}

impl<A> Default for FalconSigToStackEventHandler<A> {
    fn default() -> Self {
        Self { signer: Box::new(DefaultFalconSigner) }
    }
}

impl<A> EventHandler<A> for FalconSigToStackEventHandler<A>
where
    A: AdviceProvider,
{
    fn id(&self) -> u32 {
        EVENT_FALCON_SIG_TO_STACK
    }

    fn on_event(
        &mut self,
        process: ProcessState,
        advice_provider: &mut A,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let pub_key = process.get_stack_word(0);
        let msg = process.get_stack_word(1);

        let signature = self.signer.sign_message(pub_key, msg, advice_provider)?;

        for r in signature {
            advice_provider.push_stack(AdviceSource::Value(r))?;
        }

        Ok(())
    }
}

/// A trait for signing messages using the Falcon signature scheme.
///
/// This trait is used by [FalconSigToStackEventHandler] to sign messages using the Falcon signature
/// scheme.
///
/// It is recommended to use [dsa::falcon_sign] to implement this trait once the private key has
/// been fetched from a user-defined location.
pub trait FalconSigner<A>: Send + Sync {
    /// Signs the message using the Falcon signature scheme, and returns the signature as a
    /// `Vec<Felt>`.
    fn sign_message(
        &self,
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

impl<A> FalconSigner<A> for DefaultFalconSigner {
    fn sign_message(
        &self,
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
