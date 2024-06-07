use super::{AssemblyContext, BodyWrapper, Decorator, DecoratorList, Instruction};
use alloc::{borrow::Borrow, string::ToString, vec::Vec};
use vm_core::{
    mast::{MastForest, MastNode, MastNodeId},
    AdviceInjector, AssemblyOp, Operation,
};

// BASIC BLOCK BUILDER
// ================================================================================================

/// A helper struct for constructing SPAN blocks while compiling procedure bodies.
///
/// Operations and decorators can be added to a span builder via various `add_*()` and `push_*()`
/// methods, and then SPAN blocks can be extracted from the builder via `extract_*()` methods.
///
/// The same span builder can be used to construct many blocks. It is expected that when the last
/// SPAN block in a procedure's body is constructed `extract_final_span_into()` will be used.
#[derive(Default)]
pub struct BasicBlockBuilder {
    ops: Vec<Operation>,
    decorators: DecoratorList,
    epilogue: Vec<Operation>,
    last_asmop_pos: usize,
}

/// Constructors
impl BasicBlockBuilder {
    /// Returns a new [SpanBuilder] instantiated with the specified optional wrapper.
    ///
    /// If the wrapper is provided, the prologue of the wrapper is immediately appended to the
    /// vector of span operations. The epilogue of the wrapper is appended to the list of
    /// operations upon consumption of the builder via `extract_final_span_into()` method.
    pub(super) fn new(wrapper: Option<BodyWrapper>) -> Self {
        match wrapper {
            Some(wrapper) => Self {
                ops: wrapper.prologue,
                decorators: Vec::new(),
                epilogue: wrapper.epilogue,
                last_asmop_pos: 0,
            },
            None => Self::default(),
        }
    }
}

/// Operations
impl BasicBlockBuilder {
    /// Adds the specified operation to the list of span operations.
    pub fn push_op(&mut self, op: Operation) {
        self.ops.push(op);
    }

    /// Adds the specified sequence of operations to the list of span operations.
    pub fn push_ops<I, O>(&mut self, ops: I)
    where
        I: IntoIterator<Item = O>,
        O: Borrow<Operation>,
    {
        self.ops.extend(ops.into_iter().map(|o| *o.borrow()));
    }

    /// Adds the specified operation n times to the list of span operations.
    pub fn push_op_many(&mut self, op: Operation, n: usize) {
        let new_len = self.ops.len() + n;
        self.ops.resize(new_len, op);
    }
}

/// Decorators
impl BasicBlockBuilder {
    /// Add the specified decorator to the list of span decorators.
    pub fn push_decorator(&mut self, decorator: Decorator) {
        self.decorators.push((self.ops.len(), decorator));
    }

    /// Adds the specified advice injector to the list of span decorators.
    pub fn push_advice_injector(&mut self, injector: AdviceInjector) {
        self.push_decorator(Decorator::Advice(injector));
    }

    /// Adds an AsmOp decorator to the list of span decorators.
    ///
    /// This indicates that the provided instruction should be tracked and the cycle count for
    /// this instruction will be computed when the call to set_instruction_cycle_count() is made.
    pub fn track_instruction(&mut self, instruction: &Instruction, ctx: &AssemblyContext) {
        let context_name = ctx.unwrap_current_procedure().name().to_string();
        let num_cycles = 0;
        let op = instruction.to_string();
        let should_break = instruction.should_break();
        let op = AssemblyOp::new(context_name, num_cycles, op, should_break);
        self.push_decorator(Decorator::AsmOp(op));
        self.last_asmop_pos = self.decorators.len() - 1;
    }

    /// Computes the number of cycles elapsed since the last invocation of track_instruction()
    /// and updates the related AsmOp decorator to include this cycle count.
    ///
    /// If the cycle count is 0, the original decorator is removed from the list. This can happen
    /// for instructions which do not contribute any operations to the span block - e.g., exec,
    /// call, and syscall.
    pub fn set_instruction_cycle_count(&mut self) {
        // get the last asmop decorator and the cycle at which it was added
        let (op_start, assembly_op) =
            self.decorators.get_mut(self.last_asmop_pos).expect("no asmop decorator");
        assert!(matches!(assembly_op, Decorator::AsmOp(_)));

        // compute the cycle count for the instruction
        let cycle_count = self.ops.len() - *op_start;

        // if the cycle count is 0, remove the decorator; otherwise update its cycle count
        if cycle_count == 0 {
            self.decorators.remove(self.last_asmop_pos);
        } else if let Decorator::AsmOp(assembly_op) = assembly_op {
            assembly_op.set_num_cycles(cycle_count as u8)
        }
    }
}

/// Span Constructors
impl BasicBlockBuilder {
    /// Creates and returns a new BASIC BLOCK node from the operations and decorators currently in
    /// this builder. If the builder is empty, then no node is created and `None` is returned.
    ///
    /// This consumes all operations and decorators in the builder, but does not touch the
    /// operations in the epilogue of the builder.
    pub fn make_basic_block(&mut self, mast_forest: &mut MastForest) -> Option<MastNodeId> {
        if !self.ops.is_empty() {
            let ops = self.ops.drain(..).collect();
            let decorators = self.decorators.drain(..).collect();

            let basic_block_node = MastNode::new_basic_block_with_decorators(ops, decorators);
            let basic_block_node_id = mast_forest.add_node(basic_block_node);

            Some(basic_block_node_id)
        } else if !self.decorators.is_empty() {
            // this is a bug in the assembler. we shouldn't have decorators added without their
            // associated operations
            // TODO: change this to an error or allow decorators in empty span blocks
            unreachable!("decorators in an empty SPAN block")
        } else {
            None
        }
    }

    /// Creates and returns a new BASIC BLOCK node from the operations and decorators currently in
    /// this builder. If the builder is empty, then no node is created and `None` is returned.
    ///
    /// The main differences with [`Self::to_basic_block`] are:
    /// - Operations contained in the epilogue of the builder are appended to the list of ops which
    ///   go into the new BASIC BLOCK node.
    /// - The builder is consumed in the process.
    pub fn into_basic_block(mut self, mast_forest: &mut MastForest) -> Option<MastNodeId> {
        self.ops.append(&mut self.epilogue);
        self.make_basic_block(mast_forest)
    }
}
