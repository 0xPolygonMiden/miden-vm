#[derive(Debug, thiserror::Error)]
pub enum Ext2InttError {
    #[error("input domain size must be a power of two, but was {0}")]
    DomainSizeNotPowerOf2(u64),
    #[error("input domain size ({0} elements) is too small")]
    DomainSizeTooSmall(u64),
    #[error("address of the last input must be smaller than 2^32, but was {0}")]
    InputEndAddressTooBig(u64),
    #[error("input size must be smaller than 2^32, but was {0}")]
    InputSizeTooBig(u64),
    #[error("address of the first input must be smaller than 2^32, but was {0}")]
    InputStartAddressTooBig(u64),
    #[error("address of the first input is not word aligned: {0}")]
    InputStartNotWordAligned(u64),
    #[error("output size ({0}) cannot be greater than the input size ({1})")]
    OutputSizeTooBig(usize, usize),
    #[error("output size must be greater than 0")]
    OutputSizeIsZero,
    #[error("uninitialized memory at address {0}")]
    UninitializedMemoryAddress(u32),
}
