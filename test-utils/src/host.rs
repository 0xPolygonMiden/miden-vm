use alloc::sync::Arc;

use processor::{AdviceProvider, AdviceSource, DefaultHost, MastForest, ProcessState};
use prover::{ExecutionError, Host, MemAdviceProvider};
use stdlib::{EVENT_FALCON_SIG_TO_STACK, falcon_sign};

pub struct TestHost(DefaultHost<MemAdviceProvider>);

impl TestHost {
    pub fn new(advice_provider: MemAdviceProvider) -> Self {
        Self(DefaultHost::new(advice_provider))
    }

    pub fn load_mast_forest(&mut self, mast_forest: Arc<MastForest>) -> Result<(), ExecutionError> {
        self.0.load_mast_forest(mast_forest)
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

    fn get_mast_forest(&self, node_digest: &prover::Digest) -> Option<Arc<processor::MastForest>> {
        self.0.get_mast_forest(node_digest)
    }

    fn on_event(&mut self, process: ProcessState, event_id: u32) -> Result<(), ExecutionError> {
        if event_id == EVENT_FALCON_SIG_TO_STACK {
            let advice_provider = self.advice_provider_mut();
            push_falcon_signature(advice_provider, process)
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
///   Advice stack: [SIGNATURE]
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
) -> Result<(), ExecutionError> {
    let pub_key = process.get_stack_word(0);
    let msg = process.get_stack_word(1);

    let pk_sk = advice_provider
        .get_mapped_values(&pub_key.into())
        .ok_or(ExecutionError::AdviceMapKeyNotFound(pub_key))?;

    let result = falcon_sign(pk_sk, msg)
        .ok_or_else(|| ExecutionError::MalformedSignatureKey("RPO Falcon512"))?;

    for r in result {
        advice_provider.push_stack(AdviceSource::Value(r))?;
    }
    Ok(())
}
