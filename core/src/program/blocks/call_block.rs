use super::{fmt, hasher, Digest, Felt, Operation};

// CALL BLOCK
// ================================================================================================
/// Block for a function call.
///
/// Executes the function referenced by `fn_hash`. Fails if the body is unavailable to the VM, or
/// if the execution of the call fails.
///
/// The hash of a call block is computed as:
///
/// > hash(fn_hash || padding, domain=CALL_DOMAIN)
/// > hash(fn_hash || padding, domain=SYSCALL_DOMAIN)  # when a syscall is used
///
/// Where `fn_hash` is 4 field elements (256 bits), and `padding` is 4 ZERO elements (256 bits).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Call {
    hash: Digest,
    fn_hash: Digest,
    is_syscall: bool,
}

impl Call {
    // CONSTANTS
    // --------------------------------------------------------------------------------------------
    /// The domain of the call block (used for control block hashing).
    pub const CALL_DOMAIN: Felt = Felt::new(Operation::Call.op_code() as u64);
    /// The domain of the syscall block (used for control block hashing).
    pub const SYSCALL_DOMAIN: Felt = Felt::new(Operation::SysCall.op_code() as u64);

    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Returns a new [Call] block instantiated with the specified function body hash.
    pub fn new(fn_hash: Digest) -> Self {
        let hash = hasher::merge_in_domain(&[fn_hash, Digest::default()], Self::CALL_DOMAIN);
        Self {
            hash,
            fn_hash,
            is_syscall: false,
        }
    }

    /// Returns a new [Call] block instantiated with the specified function body hash and marked
    /// as a kernel call.
    pub fn new_syscall(fn_hash: Digest) -> Self {
        let hash = hasher::merge_in_domain(&[fn_hash, Digest::default()], Self::SYSCALL_DOMAIN);
        Self {
            hash,
            fn_hash,
            is_syscall: true,
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns a hash of this code block.
    pub fn hash(&self) -> Digest {
        self.hash
    }

    /// Returns a hash of the function to be called by this block.
    pub fn fn_hash(&self) -> Digest {
        self.fn_hash
    }

    /// Returns true if this call block corresponds to a kernel call.
    pub fn is_syscall(&self) -> bool {
        self.is_syscall
    }

    /// Returns the domain of the call block
    pub fn domain(&self) -> Felt {
        match self.is_syscall() {
            true => Self::SYSCALL_DOMAIN,
            false => Self::CALL_DOMAIN,
        }
    }
}

#[cfg(feature = "formatter")]
impl crate::prettier::PrettyPrint for Call {
    fn render(&self) -> crate::prettier::Document {
        use crate::prettier::*;

        let doc = if self.is_syscall {
            const_text("syscall")
        } else {
            const_text("call")
        };
        doc + const_text(".") + self.fn_hash.render()
    }
}

#[cfg(feature = "formatter")]
impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::prettier::PrettyPrint;
        self.pretty_print(f)
    }
}

#[cfg(not(feature = "formatter"))]
impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::utils::DisplayHex;

        if self.is_syscall {
            f.write_str("syscall.")?;
        } else {
            f.write_str("call.")?;
        }
        write!(f, "{:#}", DisplayHex(self.fn_hash.as_bytes().as_slice()))
    }
}
