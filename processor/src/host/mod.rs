use super::{ExecutionError, Felt, ProcessState};
use crate::MemAdviceProvider;
use core::fmt;
use vm_core::{crypto::merkle::MerklePath, AdviceInjector, DebugOptions, Word};

pub(super) mod advice;
use advice::{AdviceExtractor, AdviceProvider};

mod debug;

// HOST TRAIT
// ================================================================================================

/// Defines an interface by which the VM can make requests to the host. The set of requests are
/// defined by the [HostRequest] enum. The host is responsible for handling these requests and
/// returning the results to the VM in the form of [HostResponse]. The host is provided with a
/// reference to the current state of the VM ([ProcessState]), which it can use to extract the
/// data required to fulfill the request.
pub trait Host {
    fn handle_request<S: ProcessState>(
        &mut self,
        process: &S,
        function: &HostRequest,
    ) -> Result<HostResponse, ExecutionError>;

    /// Creates a "by reference" host for this instance.
    ///
    /// The returned adapter also implements [Host] and will simply mutably borrow this
    /// instance.
    fn by_ref(&mut self) -> &mut Self {
        // this trait follows the same model as
        // [io::Read](https://doc.rust-lang.org/std/io/trait.Read.html#method.by_ref).
        //
        // this approach allows the flexibility to take a host  either as owned or by mutable
        // reference - both equally compatible with the trait requirements as we implement
        // `Host` for mutable references of any type that also implements `Host`.
        self
    }
}

impl<'a, H> Host for &'a mut H
where
    H: Host,
{
    fn handle_request<S: ProcessState>(
        &mut self,
        process: &S,
        function: &HostRequest,
    ) -> Result<HostResponse, ExecutionError> {
        H::handle_request(self, process, function)
    }
}

// HOST REQUEST
// ================================================================================================

/// [HostRequest] defines the requests that can be executed by the VM against the host. Executing a
/// host function does not affect the state of the main VM components. However, host functions may
/// modify the host.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum HostRequest {
    /// Prints out information about the state of the VM based on the specified options. This
    /// decorator is executed only in debug mode.
    Debug(DebugOptions),
    /// Injects new data into the advice provider, as specified by the injector.
    PrepAdvice(AdviceInjector),
    /// Extracts data from the advice provider as specified by the extractor.
    GetAdvice(AdviceExtractor),
}

impl fmt::Display for HostRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Debug(options) => write!(f, "host::debug({options})"),
            Self::PrepAdvice(injector) => write!(f, "host::prep_advice({injector})"),
            Self::GetAdvice(extractor) => write!(f, "host::get_advice({extractor})"),
        }
    }
}

// HOST RESPONSE
// ================================================================================================

/// Response returned by the host upon successful execution of a [HostFunction].
pub enum HostResponse {
    MerklePath(MerklePath),
    DoubleWord([Word; 2]),
    Word(Word),
    Element(Felt),
    Unit,
}

// DEFAULT HOST IMPLEMENTATION
// ================================================================================================

pub struct DefaultHost<A> {
    adv_provider: A,
}

impl Default for DefaultHost<MemAdviceProvider> {
    fn default() -> Self {
        Self {
            adv_provider: MemAdviceProvider::default(),
        }
    }
}

impl<A: AdviceProvider> DefaultHost<A> {
    pub fn new(adv_provider: A) -> Self {
        Self { adv_provider }
    }

    #[cfg(any(test, feature = "internals"))]
    pub fn advice_provider(&self) -> &A {
        &self.adv_provider
    }

    #[cfg(any(test, feature = "internals"))]
    pub fn advice_provider_mut(&mut self) -> &mut A {
        &mut self.adv_provider
    }
}

impl<A: AdviceProvider> Host for DefaultHost<A> {
    fn handle_request<S: ProcessState>(
        &mut self,
        process: &S,
        function: &HostRequest,
    ) -> Result<HostResponse, ExecutionError> {
        match function {
            HostRequest::GetAdvice(extractor) => {
                self.adv_provider.hanlde_extractor_request(process, extractor)
            }
            HostRequest::PrepAdvice(injector) => {
                self.adv_provider.handle_injector_request(process, injector)
            }
            HostRequest::Debug(options) => debug::print_debug_info(process, options),
        }
    }
}
