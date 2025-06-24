use alloc::sync::Arc;

use processor::{
    AdviceProvider, AdviceSource, DefaultHost, ErrorContext, MastForest, ProcessState,
};
use prover::{ExecutionError, Host, MemAdviceProvider, Word};
use stdlib::{EVENT_FALCON_SIG_TO_STACK, falcon_sign};
use vm_core::mast::MastNodeExt;

pub struct TestHost(DefaultHost<MemAdviceProvider>);

impl TestHost {
    pub fn new(advice_provider: MemAdviceProvider) -> Self {
        Self(DefaultHost::new(advice_provider))
    }

    pub fn load_mast_forest(&mut self, mast_forest: Arc<MastForest>) -> Result<(), ExecutionError> {
        self.0.load_mast_forest(mast_forest)
    }

    pub fn advice_provider(&self) -> &MemAdviceProvider {
        self.0.advice_provider()
    }

    pub fn advice_provider_mut(&mut self) -> &mut MemAdviceProvider {
        self.0.advice_provider_mut()
    }

    pub fn into_inner(self) -> MemAdviceProvider {
        self.0.into_inner()
    }
}

impl Host for TestHost {
    type AdviceProvider = MemAdviceProvider;

    fn advice_provider(&self) -> &Self::AdviceProvider {
        self.0.advice_provider()
    }

    fn advice_provider_mut(&mut self) -> &mut Self::AdviceProvider {
        self.0.advice_provider_mut()
    }

    fn get_mast_forest(&self, node_digest: &Word) -> Option<Arc<processor::MastForest>> {
        self.0.get_mast_forest(node_digest)
    }

    fn on_event(
        &mut self,
        process: ProcessState,
        event_id: u32,
        err_ctx: &ErrorContext<'_, impl MastNodeExt>,
    ) -> Result<(), ExecutionError> {
        if event_id == EVENT_FALCON_SIG_TO_STACK {
            let advice_provider = self.advice_provider_mut();
            push_falcon_signature(advice_provider, process, err_ctx)
        } else {
            Ok(())
        }
    }
}

/// Pushes values onto the advice stack which are required for verification of a DSA in Miden
/// VM.
///
/// Inputs:
///   Operand stack: [PK, MSG, ...]
///   Advice stack: \[ SIGNATURE \]
///
/// Outputs:
///   Operand stack: [PK, MSG, ...]
///   Advice stack: [...]
///
/// Where:
/// - PK is the digest of an expanded public.
/// - MSG is the digest of the message to be signed.
/// - SIGNATURE is the signature being verified.
///
/// The advice provider is expected to contain the private key associated to the public key PK.
pub fn push_falcon_signature(
    advice_provider: &mut impl AdviceProvider,
    process: ProcessState,
    err_ctx: &ErrorContext<'_, impl MastNodeExt>,
) -> Result<(), ExecutionError> {
    let pub_key = process.get_stack_word(0);
    let msg = process.get_stack_word(1);

    let pk_sk = advice_provider
        .get_mapped_values(&pub_key)
        .ok_or(ExecutionError::advice_map_key_not_found(pub_key, err_ctx))?;

    let result = falcon_sign(pk_sk, msg)
        .ok_or_else(|| ExecutionError::malformed_signature_key("RPO Falcon512", err_ctx))?;

    for r in result {
        advice_provider.push_stack(AdviceSource::Value(r), err_ctx)?;
    }
    Ok(())
}
