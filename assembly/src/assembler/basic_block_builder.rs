use alloc::{borrow::Borrow, vec::Vec};

use vm_core::{
    Decorator, Operation,
    mast::{DecoratorId, MastNodeId},
    sys_events::SystemEvent,
};

use super::{BodyWrapper, DecoratorList, mast_forest_builder::MastForestBuilder};
use crate::AssemblyError;

// BASIC BLOCK BUILDER
// ================================================================================================

/// A helper struct for constructing basic blocks while compiling procedure bodies.
///
/// Operations and decorators can be added to a basic block builder via various `add_*()` and
/// `push_*()` methods, and then basic blocks can be extracted from the builder via `extract_*()`
/// methods.
///
/// The same basic block builder can be used to construct many blocks. It is expected that when the
/// last basic block in a procedure's body is constructed [`Self::try_into_basic_block`] will be
/// used.
#[derive(Debug)]
pub struct BasicBlockBuilder<'a> {
    ops: Vec<Operation>,
    decorators: DecoratorList,
    epilogue: Vec<Operation>,
    mast_forest_builder: &'a mut MastForestBuilder,

    // debug helpers
    last_op_count: usize,
}

/// Constructors
impl<'a> BasicBlockBuilder<'a> {
    /// Returns a new [`BasicBlockBuilder`] instantiated with the specified optional wrapper.
    ///
    /// If the wrapper is provided, the prologue of the wrapper is immediately appended to the
    /// vector of span operations. The epilogue of the wrapper is appended to the list of operations
    /// upon consumption of the builder via the [`Self::try_into_basic_block`] method.
    pub(super) fn new(
        wrapper: Option<BodyWrapper>,
        mast_forest_builder: &'a mut MastForestBuilder,
    ) -> Self {
        match wrapper {
            Some(wrapper) => {
                let ops = wrapper.prologue;
                let last_op_count = ops.len();

                Self {
                    ops,
                    decorators: Vec::new(),
                    epilogue: wrapper.epilogue,
                    mast_forest_builder,
                    last_op_count,
                }
            },
            None => Self {
                ops: Default::default(),
                decorators: Default::default(),
                epilogue: Default::default(),
                mast_forest_builder,
                last_op_count: 0,
            },
        }
    }
}

/// Accessors
impl BasicBlockBuilder<'_> {
    /// Returns a reference to the internal [`MastForestBuilder`].
    pub fn mast_forest_builder(&self) -> &MastForestBuilder {
        self.mast_forest_builder
    }

    /// Returns a mutable reference to the internal [`MastForestBuilder`].
    pub fn mast_forest_builder_mut(&mut self) -> &mut MastForestBuilder {
        self.mast_forest_builder
    }
}

/// Operations
impl BasicBlockBuilder<'_> {
    /// Adds the specified operation to the list of basic block operations.
    pub fn push_op(&mut self, op: Operation) {
        self.ops.push(op);
    }

    /// Adds the specified sequence of operations to the list of basic block operations.
    pub fn push_ops<I, O>(&mut self, ops: I)
    where
        I: IntoIterator<Item = O>,
        O: Borrow<Operation>,
    {
        self.ops.extend(ops.into_iter().map(|o| *o.borrow()));
    }

    /// Adds the specified operation n times to the list of basic block operations.
    pub fn push_op_many(&mut self, op: Operation, n: usize) {
        let new_len = self.ops.len() + n;
        self.ops.resize(new_len, op);
    }

    /// Converts the system event into its corresponding event ID, and adds an `Emit` operation
    /// to the list of basic block operations.
    pub fn push_system_event(&mut self, sys_event: SystemEvent) {
        self.push_op(Operation::Emit(sys_event.into_event_id()))
    }

    /// Returns the number of operations added to the builder since the last call to this method,
    /// and resets the counter.
    pub fn get_op_count_and_reset(&mut self) -> u8 {
        let op_count: u8 = (self.ops.len() - self.last_op_count)
            .try_into()
            .expect("instruction compiles down to more than 255 VM operations");
        self.last_op_count = self.ops.len();

        op_count
    }
}

/// Decorators
impl BasicBlockBuilder<'_> {
    /// Add the specified decorator to the list of basic block decorators.
    pub fn push_decorator(&mut self, decorator: Decorator) -> Result<(), AssemblyError> {
        let decorator_id = self.mast_forest_builder.ensure_decorator(decorator)?;
        self.decorators.push((self.ops.len(), decorator_id));

        Ok(())
    }
}

/// Span Constructors
impl BasicBlockBuilder<'_> {
    /// Creates and returns a new basic block node from the operations and decorators currently in
    /// this builder.
    ///
    /// If there are no operations however, then no node is created, the decorators are left
    /// untouched and `None` is returned. Use [`Self::drain_decorators`] to retrieve the decorators
    /// in this case.
    ///
    /// This consumes all operations in the builder, but does not touch the operations in the
    /// epilogue of the builder.
    pub fn make_basic_block(&mut self) -> Result<Option<MastNodeId>, AssemblyError> {
        self.last_op_count = 0;

        if !self.ops.is_empty() {
            let ops = self.ops.drain(..).collect();
            let decorators = if !self.decorators.is_empty() {
                Some(self.decorators.drain(..).collect())
            } else {
                None
            };

            let basic_block_node_id = self.mast_forest_builder.ensure_block(ops, decorators)?;

            Ok(Some(basic_block_node_id))
        } else {
            Ok(None)
        }
    }

    /// Creates and returns a new basic block node from the operations and decorators currently in
    /// this builder. If there are no operations however, we return the decorators that were
    /// accumulated up until this point. If the builder is empty, then no node is created and
    /// `Nothing` is returned.
    ///
    /// The main differences with [`Self::make_basic_block`] are:
    /// - Operations contained in the epilogue of the builder are appended to the list of ops which
    ///   go into the new BASIC BLOCK node.
    /// - The builder is consumed in the process.
    /// - Hence, any remaining decorators if no basic block was created are drained and returned.
    pub fn try_into_basic_block(mut self) -> Result<BasicBlockOrDecorators, AssemblyError> {
        self.ops.append(&mut self.epilogue);

        if let Some(basic_block_node_id) = self.make_basic_block()? {
            Ok(BasicBlockOrDecorators::BasicBlock(basic_block_node_id))
        } else if let Some(decorator_ids) = self.drain_decorators() {
            Ok(BasicBlockOrDecorators::Decorators(decorator_ids))
        } else {
            Ok(BasicBlockOrDecorators::Nothing)
        }
    }

    /// Drains and returns the decorators in the builder, if any.
    ///
    /// This should only be called after [`Self::make_basic_block`], when no blocks were created.
    /// In other words, there MUST NOT be any operations left in the builder when this is called.
    ///
    /// # Panics
    ///
    /// Panics if there are still operations left in the builder.
    pub fn drain_decorators(&mut self) -> Option<Vec<DecoratorId>> {
        assert!(self.ops.is_empty());
        if !self.decorators.is_empty() {
            Some(self.decorators.drain(..).map(|(_, decorator_id)| decorator_id).collect())
        } else {
            None
        }
    }
}

/// Holds either the node id of a basic block, or a list of decorators that are currently not
/// attached to any node.
pub enum BasicBlockOrDecorators {
    BasicBlock(MastNodeId),
    Decorators(Vec<DecoratorId>),
    Nothing,
}
