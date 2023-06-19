pub use crate::crypto::falcon_rust::binder::rpo128_context;
pub use crate::crypto::falcon_rust::binder::shake256_context;
use crate::crypto::falcon_rust::binder::*;
use libc::c_void;

// Wrappers for unsafe functions
impl shake256_context {
    /// Initializing an RNG from seed.
    pub fn init_with_seed(seed: &[u8]) -> Self {
        let mut ctx = shake256_context {
            opaque_contents: [0u64; 26],
        };
        unsafe {
            shake256_init_prng_from_seed(
                &mut ctx as *mut shake256_context,
                seed.as_ptr() as *const c_void,
                seed.len() as u64,
            );
        }
        ctx
    }

    /// Finalize the RNG
    pub fn finalize(&mut self) {
        unsafe { shake256_flip(self as *mut shake256_context) }
    }
}

// Wrappers for unsafe functions
impl rpo128_context {
    /// Initializing an RNG.
    pub fn init() -> Self {
        let mut ctx = rpo128_context {
            opaque_contents: [0u64; 13],
        };
        unsafe {
            rpo128_init(&mut ctx as *mut rpo128_context);
        }
        ctx
    }

    /// Inject data to the RNG
    pub fn inject(&mut self, data: &[u8]) {
        unsafe {
            rpo128_inject(
                self as *mut rpo128_context,
                data.as_ptr() as *const c_void,
                data.len() as u64,
            )
        }
    }

    /// Finalize the RNG
    pub fn finalize(&mut self) {
        unsafe { rpo128_flip(self as *mut rpo128_context) }
    }

    /// Extract data from the RNG
    pub fn extract(&mut self, len: usize) -> Vec<u8> {
        let data = vec![0u8; len];
        unsafe {
            rpo128_extract(self as *mut rpo128_context, data.as_ptr() as *mut c_void, len as u64);
        }
        data
    }
}
