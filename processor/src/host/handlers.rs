use alloc::boxed::Box;
use std::error::Error;

use vm_core::DebugOptions;

use crate::{ExecutionError, ProcessState};

// EVENT HANDLER TRAIT
// ================================================================================================

/// An [`EventHandler`] defines a function that that can be called from the processor, which can
/// read the current execution state and modify the advice.
///
/// A struct implementing this trait can access its own state, but any output it produces must
/// be stored in the process's advice provider.
///
/// Stateless event handlers can be implemented both as closures or free functions with a single
/// argument `&mut ProcessState.`.
pub trait EventHandler {
    /// Handles the event when triggered.
    fn on_event(&self, process: &mut ProcessState) -> Result<(), EventError>;
}

/// Default implementation for both free functions and closures with a single
/// `&mut ProcessState.` argument.
impl<F> EventHandler for F
where
    F: Fn(&mut ProcessState) -> Result<(), EventError> + 'static,
{
    fn on_event(&self, process: &mut ProcessState) -> Result<(), EventError> {
        self(process)
    }
}

// EVENT ERROR
// ================================================================================================

/// A generic [`Error`] wrapper allowing handlers to return errors to the [`Host`](crate::Host)
/// caller.
///
/// Error handlers can define their own [`Error`] type which can be seamlessly converted
/// into this type as follows:
/// ```rust, ignore
/// pub struct MyError{ /* ... */ };
///
/// impl From<MyError> for EventError {
///     fn from(value: MyError) -> Self {
///         value.into()
///     }
/// }
/// ```
///
/// The custom handler can then use `?` as usual.
/// ```rust, ignore
/// fn try_something() -> Result<(), MyError> { /* ... */ Ok(())  }
///
/// fn my_handler(_process: &mut ProcessState) -> Result<(), EventError> {
///     // ...
///     try_something()?;
///     // ...
///     Ok(())
/// }
/// ```
pub type EventError = Box<dyn Error + Send + Sync + 'static>;

// DEBUG HANDLER
// ================================================================================================

/// Handler for debug and trace operations
pub trait DebugHandler {
    /// This function is invoked when the `Debug` decorator is executed.
    fn on_debug(
        &mut self,
        process: &mut ProcessState,
        options: &DebugOptions,
    ) -> Result<(), ExecutionError> {
        let _ = (&process, options);
        #[cfg(feature = "std")]
        crate::host::debug::print_debug_info(process, options);
        Ok(())
    }

    /// This function is invoked when the `Trace` decorator is executed.
    fn on_trace(
        &mut self,
        process: &mut ProcessState,
        trace_id: u32,
    ) -> Result<(), ExecutionError> {
        let _ = (&process, trace_id);
        #[cfg(feature = "std")]
        std::println!(
            "Trace with id {} emitted at step {} in context {}",
            trace_id,
            process.clk(),
            process.ctx()
        );
        Ok(())
    }
}
