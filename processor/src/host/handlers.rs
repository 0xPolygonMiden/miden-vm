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
    /// While this function *may* modify it's own state, it is recommended to use the
    /// `AdviceProvider` instead, and define the handler as a free function wrapped in a
    /// [`StatelessHandler`].
    fn on_event(
        &mut self,
        advice_provider: &mut dyn AdviceProvider,
        process: ProcessState,
    ) -> Result<(), EventError>;
}

// DEBUG HANDLER
// ================================================================================================

/// Handler for debug and trace operations
/// TODO: Should we merge into a single handler?
pub trait DebugHandler {
    /// TODO: What kind of error should we return
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

    fn on_trace(
        &mut self,
        _advice: &dyn AdviceProvider,
        _process: ProcessState,
        _trace_id: u32,
    ) -> Result<(), ExecutionError> {
        #[cfg(feature = "std")]
        std::println!(
            "Trace with id {} emitted at step {} in context {}",
            _trace_id,
            _process.clk(),
            _process.ctx()
        );
        Ok(())
    }
}

/// Concrete [`DebugHandler`] which re-uses the provided `on_debug` and `on_trace` implementations.
#[derive(Clone, Default)]
pub struct DefaultDebugHandler;

impl DebugHandler for DefaultDebugHandler {}

// STATELESS HANDLER
// ================================================================================================

#[derive(Clone)]
pub struct StatelessHandler<F> {
    id: u32,
    handler: F,
}

impl<F> StatelessHandler<F> {
    pub fn new(id: u32, handler: F) -> Self {
        Self { id, handler }
    }
}

pub fn trivial_handler(id: u32) -> Box<dyn EventHandler> {
    pub fn trivial_event_handler(
        _advice: &mut dyn AdviceProvider,
        _process: ProcessState,
    ) -> Result<(), EventError> {
        Ok(())
    }
    Box::new(StatelessHandler::new(id, trivial_event_handler))
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

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct EventError(#[from] Box<dyn Error + Send + Sync + 'static>);

impl EventError {
    pub fn from<E: Error + Send + Sync + 'static>(value: E) -> Self {
        Self(Box::new(value))
    }
}
