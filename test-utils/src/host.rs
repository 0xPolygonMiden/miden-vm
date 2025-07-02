use alloc::sync::Arc;

use processor::{DefaultHost, ErrorContext, MastForest, ProcessState};
use prover::{ExecutionError, Host, Word};
use stdlib::{EVENT_FALCON_SIG_TO_STACK, falcon_sign};

#[derive(Default)]
pub struct TestHost {
    host: DefaultHost,
}

impl TestHost {
    pub fn load_mast_forest(&mut self, mast_forest: Arc<MastForest>) -> Result<(), ExecutionError> {
        self.host.load_mast_forest(mast_forest)
    }
}

impl Host for TestHost {
    fn get_mast_forest(&self, node_digest: &Word) -> Option<Arc<MastForest>> {
        self.host.get_mast_forest(node_digest)
    }

    fn on_event(
        &mut self,
        process: &mut ProcessState,
        event_id: u32,
        err_ctx: &impl ErrorContext,
    ) -> Result<(), ExecutionError> {
        if event_id == EVENT_FALCON_SIG_TO_STACK {
            push_falcon_signature(process, err_ctx)
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
    process: &mut ProcessState,
    err_ctx: &impl ErrorContext,
) -> Result<(), ExecutionError> {
    let pub_key = process.get_stack_word(0);
    let msg = process.get_stack_word(1);

    let pk_sk = process
        .advice_provider()
        .get_mapped_values(&pub_key)
        .map_err(|err| ExecutionError::advice_error(err, process.clk(), err_ctx))?;

    let result = falcon_sign(pk_sk, msg)
        .ok_or_else(|| ExecutionError::malformed_signature_key("RPO Falcon512", err_ctx))?;

    for r in result {
        process.advice_provider_mut().push_stack(r);
    }
    Ok(())
}
