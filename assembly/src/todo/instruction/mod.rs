mod field_ops;
mod flow_ops;

use vm_core::{code_blocks::CodeBlock, Operation};

use crate::{parsers::Instruction, AssemblerError};

use super::{Assembler, AssemblerContext, SpanBuilder};

impl Assembler {
    pub(super) fn compile_instruction(
        &self,
        context: &mut AssemblerContext,
        span: &mut SpanBuilder,
        instruction: &Instruction,
    ) -> Result<Option<CodeBlock>, AssemblerError> {
        use Operation::*;

        match instruction {
            Instruction::Assert => span.add_op(Assert),
            Instruction::AssertEq => span.add_ops(&[Eq, Assert]),
            Instruction::Assertz => span.add_ops(&[Eqz, Assert]),
            Instruction::Add => span.add_op(Add),
            Instruction::AddImm(imm) => field_ops::add_imm(imm, span),
            Instruction::Sub => span.add_ops(&[Neg, Add]),
            Instruction::SubImm(imm) => span.add_ops(&[Push(-*imm), Add]),
            Instruction::PushConstants(imms) => span.add_ops(imms.iter().copied().map(Push)),
            Instruction::ExecLocal(idx) => flow_ops::exec_local(*idx, context),
            Instruction::ExecImported(id) => flow_ops::exec_imported(id, self, context),
            Instruction::CallLocal(idx) => flow_ops::call_local(*idx, context),
            Instruction::CallImported(id) => flow_ops::call_imported(id, self, context),
            _ => todo!(),
        }
    }
}
