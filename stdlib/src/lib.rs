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

mod dsa;

// STANDARD LIBRARY
// ================================================================================================

/// TODO: add docs
#[derive(Clone)]
pub struct StdLibrary(Library);

impl AsRef<Library> for StdLibrary {
    fn as_ref(&self) -> &Library {
        &self.0
    }
}

impl From<StdLibrary> for Library {
    fn from(value: StdLibrary) -> Self {
        value.0
    }
}

impl StdLibrary {
    /// Serialized representation of the Miden standard library.
    pub const SERIALIZED: &'static [u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/assets/std.masl"));

    /// Returns a reference to the [MastForest] underlying the Miden standard library.
    pub fn mast_forest(&self) -> &Arc<MastForest> {
        self.0.mast_forest()
    }
}

impl Default for StdLibrary {
    fn default() -> Self {
        static STDLIB: LazyLock<StdLibrary> = LazyLock::new(|| {
            let contents =
                Library::read_from_bytes(StdLibrary::SERIALIZED).expect("failed to read std masl!");
            StdLibrary(contents)
        });
        STDLIB.clone()
    }
}

impl HostLibrary for StdLibrary {
    fn get_event_handlers<A>(&self) -> Vec<Box<dyn EventHandler<A>>>
    where
        A: AdviceProvider + 'static,
    {
        // TODO(plafer): add `with_falcon_sig_handler()` method to `StdLibrary` to allow customizing
        // how Falcon signatures are handled
        vec![Box::new(FalconSigToStackHandler::default())]
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

pub struct FalconSigToStackHandler<A> {
    signature_handler: Box<dyn FalconSigHandler<A>>,
}

impl<A> Default for FalconSigToStackHandler<A> {
    fn default() -> Self {
        Self {
            signature_handler: Box::new(DefaultFalconSigHandler),
        }
    }
}

impl<A> EventHandler<A> for FalconSigToStackHandler<A>
where
    A: AdviceProvider,
{
    fn id(&self) -> u32 {
        EVENT_FALCON_SIG_TO_STACK
    }

    // TODO(plafer): Probably best to have a specific error type for this
    fn on_event(
        &mut self,
        process: ProcessState,
        advice_provider: &mut A,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let pub_key = process.get_stack_word(0);
        let msg = process.get_stack_word(1);

        let signature = self.signature_handler.handle_signature(pub_key, msg, advice_provider)?;

        for r in signature {
            advice_provider.push_stack(AdviceSource::Value(r))?;
        }

        Ok(())
    }
}

// TODO(plafer): double check if this is the correct abstraction
pub trait FalconSigHandler<A> {
    fn handle_signature(
        &self,
        pub_key: Word,
        msg: Word,
        advice_provider: &A,
    ) -> Result<Vec<Felt>, Box<dyn Error + Send + Sync + 'static>>
    where
        A: AdviceProvider;
}

struct DefaultFalconSigHandler;

impl<A> FalconSigHandler<A> for DefaultFalconSigHandler {
    fn handle_signature(
        &self,
        pub_key: Word,
        msg: Word,
        advice_provider: &A,
    ) -> Result<Vec<Felt>, Box<dyn Error + Send + Sync + 'static>>
    where
        A: AdviceProvider,
    {
        let pk_sk = advice_provider
            .get_mapped_values(&pub_key.into())
            .ok_or(AdviceProviderError::AdviceMapKeyNotFound(pub_key))?;

        dsa::falcon_sign(pk_sk, msg)
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
        let exists = stdlib.0.module_infos().any(|module| {
            module
                .procedures()
                .any(|(_, proc)| module.path().clone().append(&proc.name).unwrap() == path)
        });

        assert!(exists);
    }
}
