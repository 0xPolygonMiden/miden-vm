use core::fmt;

use super::{AstFormatterContext, FormattableCodeBody, Instruction, Node};

// FORMATTING OF NODES
// ================================================================================================
pub struct FormattableNode<'a> {
    node: &'a Node,
    context: &'a AstFormatterContext<'a>,
}

impl<'a> FormattableNode<'a> {
    pub fn new(node: &'a Node, context: &'a AstFormatterContext<'a>) -> Self {
        Self { node, context }
    }
}

impl fmt::Display for FormattableNode<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.node {
            Node::Instruction(i) => {
                write!(f, "{}", FormattableInstruction::new(i, self.context))
            }
            Node::IfElse {
                true_case,
                false_case,
            } => {
                self.context.indent(f)?;
                writeln!(f, "if.true")?;
                write!(
                    f,
                    "{}",
                    FormattableCodeBody::new(true_case, &self.context.inner_scope_context())
                )?;
                if !false_case.nodes().is_empty() {
                    // No false branch - don't output else branch
                    self.context.indent(f)?;
                    writeln!(f, "else")?;

                    write!(
                        f,
                        "{}",
                        FormattableCodeBody::new(false_case, &self.context.inner_scope_context())
                    )?;
                }
                self.context.indent(f)?;
                writeln!(f, "end")
            }
            Node::Repeat { times, body } => {
                self.context.indent(f)?;
                writeln!(f, "repeat.{times}")?;

                write!(
                    f,
                    "{}",
                    FormattableCodeBody::new(body, &self.context.inner_scope_context())
                )?;

                self.context.indent(f)?;
                writeln!(f, "end")
            }
            Node::While { body } => {
                self.context.indent(f)?;
                writeln!(f, "while.true")?;

                write!(
                    f,
                    "{}",
                    FormattableCodeBody::new(body, &self.context.inner_scope_context())
                )?;

                self.context.indent(f)?;
                writeln!(f, "end")
            }
        }
    }
}

// FORMATTING OF INSTRUCTIONS WITH INDENTATION
// ================================================================================================
pub struct FormattableInstruction<'a> {
    instruction: &'a Instruction,
    context: &'a AstFormatterContext<'a>,
}

impl<'a> FormattableInstruction<'a> {
    pub fn new(instruction: &'a Instruction, context: &'a AstFormatterContext<'a>) -> Self {
        Self {
            instruction,
            context,
        }
    }
}

impl fmt::Display for FormattableInstruction<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.context.indent(f)?;
        match self.instruction {
            // procedure calls are represented by indices or hashes, so must be handled specially
            Instruction::ExecLocal(index) => {
                let proc_name = self.context.local_proc(*index as usize);
                write!(f, "exec.{proc_name}")?;
            }
            Instruction::CallLocal(index) => {
                let proc_name = self.context.local_proc(*index as usize);
                write!(f, "call.{proc_name}")?;
            }
            Instruction::ExecImported(proc_id) => {
                let (_, path) = self.context.imported_proc(proc_id);
                write!(f, "exec.{path}")?;
            }
            Instruction::CallImported(proc_id) => {
                let (_, path) = self.context.imported_proc(proc_id);
                write!(f, "call.{path}")?;
            }
            Instruction::SysCall(proc_id) => {
                let (_, path) = self.context.imported_proc(proc_id);
                write!(f, "syscall.{path}")?;
            }
            Instruction::CallMastRoot(root) => {
                write!(f, "call.")?;
                display_hex_bytes(f, &root.as_bytes())?;
            }
            Instruction::ProcRefLocal(index) => {
                let proc_name = self.context.local_proc(*index as usize);
                write!(f, "procref.{proc_name}")?;
            }
            Instruction::ProcRefImported(proc_id) => {
                let (_, path) = self.context.imported_proc(proc_id);
                write!(f, "procref.{path}")?;
            }
            _ => {
                // Not a procedure call. Use the normal formatting
                write!(f, "{}", self.instruction)?;
            }
        }
        writeln!(f)
    }
}

// HELPER FUNCTIONS
// ================================================================================================

/// Builds a hex string from a byte slice
pub fn display_hex_bytes(f: &mut fmt::Formatter<'_>, bytes: &[u8]) -> fmt::Result {
    write!(f, "0x")?;
    for byte in bytes {
        write!(f, "{byte:02x}")?;
    }
    Ok(())
}

/// Builds a string from input vector to display push operation
pub fn display_push_vec<T: fmt::Display>(f: &mut fmt::Formatter<'_>, values: &[T]) -> fmt::Result {
    write!(f, "push")?;
    for elem in values {
        write!(f, ".{elem}")?;
    }
    Ok(())
}
