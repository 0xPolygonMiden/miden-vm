use super::{fmt, hasher, Digest, Felt, Operation};

// CALL BLOCK
// ================================================================================================
/// A code block describing a function call.
///
/// When the VM executes a Call block, it simply executes the code of the underlying function.
/// Thus, to execute a function call, the VM must have access to the function's body, otherwise,
/// the execution fails.
///
/// Hash of a Call block is computed by hashing a concatenation of the function's body hash with
/// zero.
#[derive(Clone, Debug)]
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

impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_syscall {
            write!(f, "syscall.0x")?;
        } else {
            write!(f, "call.0x")?;
        }

        let fn_hash_bytes: [u8; 32] = self.fn_hash.into();
        for byte in fn_hash_bytes {
            write!(f, "{byte:02x}")?;
        }

        Ok(())
    }
}
