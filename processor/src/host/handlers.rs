use alloc::boxed::Box;
use core::error::Error;

use vm_core::DebugOptions;

use crate::{AdviceProvider, ExecutionError, ProcessState};

// HANDLER TRAIT
// ================================================================================================

/// An `EventHandler` defines a function that that can be called from the processor, which can
/// read the current execution state and modify the advice, to which an output can be piped.
pub trait EventHandler {
    /// Returns the event ID this handler responds to. It corresponds to the argument
    /// given to the `emit` op code.
    fn id(&self) -> u32;

    /// Handles the event when triggered.
    ///
    /// While this function *may* modify it's own state, the same can usually be acheived using
    /// a "stateless" handler which stores it's state in the `AdviceProvider` instead.
    /// Such handlers can be instantiated using [`new_handler`].
    fn on_event(
        &mut self,
        advice_provider: &mut dyn AdviceProvider,
        process: ProcessState,
    ) -> Result<(), EventError>;
}

// DEBUG HANDLER
// ================================================================================================

/// Handler for debug and trace operations
pub trait DebugHandler {
    /// This function is invoked when the `Debug` decorator is executed.
    fn on_debug(
        &mut self,
        advice: &dyn AdviceProvider,
        process: ProcessState,
        options: &DebugOptions,
    ) -> Result<(), ExecutionError> {
        #[cfg(feature = "std")]
        {
            use crate::host::debug::Printer;
            let printer = Printer::new(process.clk(), process.ctx(), process.fmp());
            match options {
                DebugOptions::StackAll => {
                    printer.print_vm_stack(process, None);
                },
                DebugOptions::StackTop(n) => {
                    printer.print_vm_stack(process, Some(*n as usize));
                },
                DebugOptions::MemAll => {
                    printer.print_mem_all(process);
                },
                DebugOptions::MemInterval(n, m) => {
                    printer.print_mem_interval(process, *n, *m);
                },
                DebugOptions::LocalInterval(n, m, num_locals) => {
                    printer.print_local_interval(
                        process,
                        (*n as u32, *m as u32),
                        *num_locals as u32,
                    );
                },
                DebugOptions::AdvStackTop(length) => {
                    printer.print_vm_adv_stack(advice, *length as usize);
                },
            }
        }
        let _ = (advice, process, options);
        Ok(())
    }

    /// This function is invoked when the `Trace` decorator is executed.
    fn on_trace(
        &mut self,
        advice: &dyn AdviceProvider,
        process: ProcessState,
        trace_id: u32,
    ) -> Result<(), ExecutionError> {
        #[cfg(feature = "std")]
        {
            std::println!(
                "Trace with id {} emitted at step {} in context {}",
                trace_id,
                process.clk(),
                process.ctx()
            );
        }
        let _ = (advice, process, trace_id);
        Ok(())
    }
}

/// Concrete [`DebugHandler`] which re-uses the default `on_debug` and `on_trace` implementations.
#[derive(Clone, Default)]
pub struct DefaultDebugHandler;

impl DebugHandler for DefaultDebugHandler {}

// STATELESS HANDLER
// ================================================================================================

/// Returns a new stateless [`EventHandler`] which can be loaded into a [`DefaultHost`].
/// ```rust, ignore
/// host.load_handler(new_handler(id, free_handler_func));
/// host.load_handler(new_handler(id, |advice, process| { ... }));
/// ```
pub fn new_handler<F>(id: u32, handler: F) -> Box<dyn EventHandler>
where
    F: Fn(&mut dyn AdviceProvider, ProcessState) -> Result<(), EventError> + 'static,
{
    Box::new(StatelessHandler { id, handler })
}

/// Trivial event handler which does nothing.
/// ```rust, ignore
/// host.load_handler(trivial_handler(id));
/// ```
pub fn trivial_handler(id: u32) -> Box<dyn EventHandler> {
    fn trivial_handler(
        _advice: &mut dyn AdviceProvider,
        _process: ProcessState,
    ) -> Result<(), EventError> {
        Ok(())
    }

    new_handler(id, trivial_handler)
}

/// Wrapper for a stateless event handler, constructed with [`new_handler`].
#[derive(Clone)]
struct StatelessHandler<F> {
    id: u32,
    handler: F,
}

impl<F> EventHandler for StatelessHandler<F>
where
    F: Fn(&mut dyn AdviceProvider, ProcessState) -> Result<(), EventError> + 'static,
{
    fn id(&self) -> u32 {
        self.id
    }

    fn on_event(
        &mut self,
        advice_provider: &mut dyn AdviceProvider,
        process: ProcessState,
    ) -> Result<(), EventError> {
        (self.handler)(advice_provider, process)
    }
}

// EVENT ERROR
// ================================================================================================

/// A generic [`Error`] wrapper allowing handlers to return errors to the [`Host`] caller.
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct EventError(#[from] Box<dyn Error + Send + Sync + 'static>);

impl EventError {
    pub fn from<E: Error + Send + Sync + 'static>(value: E) -> Self {
        Self(Box::new(value))
    }
}
