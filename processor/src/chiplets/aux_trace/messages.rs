use core::fmt::{Display, Formatter, Result as FmtResult};

use miden_air::trace::chiplets::{hasher, kernel_rom::KERNEL_PROC_LABEL};
use vm_core::{
    utils::range, Felt, FieldElement, ONE, OPCODE_CALL, OPCODE_JOIN, OPCODE_LOOP, OPCODE_SPLIT,
};

use super::build_value;
use crate::debug::BusMessage;

// HASHER MESSAGES
// ===============================================================================================

pub struct ControlBlockRequestMessage {
    pub transition_label: Felt,
    pub addr_next: Felt,
    pub op_code: Felt,
    pub decoder_hasher_state: [Felt; 8],
}

impl<E> BusMessage<E> for ControlBlockRequestMessage
where
    E: FieldElement<BaseField = Felt>,
{
    fn value(&self, alphas: &[E]) -> E {
        let header = alphas[0]
            + alphas[1].mul_base(self.transition_label)
            + alphas[2].mul_base(self.addr_next);

        header
            + alphas[5].mul_base(self.op_code)
            + build_value(&alphas[8..16], self.decoder_hasher_state)
    }

    fn source(&self) -> &str {
        let op_code = self.op_code.as_int() as u8;
        match op_code {
            OPCODE_JOIN => "join",
            OPCODE_SPLIT => "split",
            OPCODE_LOOP => "loop",
            OPCODE_CALL => "call",
            _ => panic!("unexpected opcode: {op_code}"),
        }
    }
}

impl Display for ControlBlockRequestMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{{ transition_label: {}, addr_next: {}, op_code: {}, decoder_hasher_state: {:?} }}",
            self.transition_label, self.addr_next, self.op_code, self.decoder_hasher_state
        )
    }
}

const NUM_HEADER_ALPHAS: usize = 4;

pub struct HasherMessage {
    pub transition_label: Felt,
    pub addr_next: Felt,
    pub node_index: Felt,
    pub hasher_state: [Felt; hasher::STATE_WIDTH],
    pub source: &'static str,
}

impl<E> BusMessage<E> for HasherMessage
where
    E: FieldElement<BaseField = Felt>,
{
    fn value(&self, alphas: &[E]) -> E {
        let header = alphas[0]
            + alphas[1].mul_base(self.transition_label)
            + alphas[2].mul_base(self.addr_next)
            + alphas[3].mul_base(self.node_index);

        header
            + build_value(&alphas[range(NUM_HEADER_ALPHAS, hasher::STATE_WIDTH)], self.hasher_state)
    }

    fn source(&self) -> &str {
        self.source
    }
}

impl Display for HasherMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{{ transition_label: {}, addr_next: {}, node_index: {}, decoder_hasher_state: {:?} }}",
            self.transition_label, self.addr_next, self.node_index, self.hasher_state
        )
    }
}

// KERNEL ROM MESSAGES
// ===============================================================================================

pub struct KernelRomMessage {
    pub kernel_proc_digest: [Felt; 4],
}

impl<E> BusMessage<E> for KernelRomMessage
where
    E: FieldElement<BaseField = Felt>,
{
    fn value(&self, alphas: &[E]) -> E {
        alphas[0]
            + alphas[1].mul_base(KERNEL_PROC_LABEL)
            + alphas[2].mul_base(self.kernel_proc_digest[0])
            + alphas[3].mul_base(self.kernel_proc_digest[1])
            + alphas[4].mul_base(self.kernel_proc_digest[2])
            + alphas[5].mul_base(self.kernel_proc_digest[3])
    }

    fn source(&self) -> &str {
        "kernel rom"
    }
}

impl Display for KernelRomMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{{ proc digest: {:?} }}", self.kernel_proc_digest)
    }
}

// SPAN BLOCK MESSAGE
// ===============================================================================================

// TODO(plafer): Remove this in favor of `ControlBlockRequestMessage`
pub struct SpanBlockMessage {
    pub transition_label: Felt,
    pub addr_next: Felt,
    pub state: [Felt; 8],
}

impl<E> BusMessage<E> for SpanBlockMessage
where
    E: FieldElement<BaseField = Felt>,
{
    fn value(&self, alphas: &[E]) -> E {
        let header = alphas[0]
            + alphas[1].mul_base(self.transition_label)
            + alphas[2].mul_base(self.addr_next);

        header + build_value(&alphas[8..16], self.state)
    }

    fn source(&self) -> &str {
        "span"
    }
}

impl Display for SpanBlockMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{{ transition_label: {}, addr_next: {}, state: {:?} }}",
            self.transition_label, self.addr_next, self.state
        )
    }
}

// RESPAN BLOCK MESSAGE
// ===============================================================================================

pub struct RespanBlockMessage {
    pub transition_label: Felt,
    pub addr_next: Felt,
    pub state: [Felt; 8],
}

impl<E> BusMessage<E> for RespanBlockMessage
where
    E: FieldElement<BaseField = Felt>,
{
    fn value(&self, alphas: &[E]) -> E {
        let header = alphas[0]
            + alphas[1].mul_base(self.transition_label)
            + alphas[2].mul_base(self.addr_next - ONE);

        header + build_value(&alphas[8..16], self.state)
    }

    fn source(&self) -> &str {
        "respan"
    }
}

impl Display for RespanBlockMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{{ transition_label: {}, addr_next: {}, state: {:?} }}",
            self.transition_label, self.addr_next, self.state
        )
    }
}

// END BLOCK MESSAGE
// ===============================================================================================

pub struct EndBlockMessage {
    pub addr: Felt,
    pub transition_label: Felt,
    pub digest: [Felt; 4],
}

impl<E> BusMessage<E> for EndBlockMessage
where
    E: FieldElement<BaseField = Felt>,
{
    fn value(&self, alphas: &[E]) -> E {
        let header =
            alphas[0] + alphas[1].mul_base(self.transition_label) + alphas[2].mul_base(self.addr);

        header + build_value(&alphas[8..12], self.digest)
    }

    fn source(&self) -> &str {
        "end"
    }
}

impl Display for EndBlockMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{{ addr: {}, transition_label: {}, digest: {:?} }}",
            self.addr, self.transition_label, self.digest
        )
    }
}

// BITWISE REQUEST MESSAGE
// ===============================================================================================

pub struct BitwiseMessage {
    pub op_label: Felt,
    pub a: Felt,
    pub b: Felt,
    pub z: Felt,
}

impl<E> BusMessage<E> for BitwiseMessage
where
    E: FieldElement<BaseField = Felt>,
{
    fn value(&self, alphas: &[E]) -> E {
        alphas[0] + build_value(&alphas[1..5], [self.op_label, self.a, self.b, self.z])
    }

    fn source(&self) -> &str {
        "bitwise"
    }
}

impl Display for BitwiseMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{{ op_label: {}, a: {}, b: {}, z: {} }}",
            self.op_label, self.a, self.b, self.z
        )
    }
}

// MEMORY REQUEST WORD MESSAGE
// ===============================================================================================

pub struct MemRequestWordMessage {
    pub op_label: Felt,
    pub ctx: Felt,
    pub addr: Felt,
    pub clk: Felt,
    pub word: [Felt; 4],
    pub source: &'static str,
}

impl<E> BusMessage<E> for MemRequestWordMessage
where
    E: FieldElement<BaseField = Felt>,
{
    fn value(&self, alphas: &[E]) -> E {
        alphas[0]
            + build_value(
                &alphas[1..9],
                [
                    self.op_label,
                    self.ctx,
                    self.addr,
                    self.clk,
                    self.word[0],
                    self.word[1],
                    self.word[2],
                    self.word[3],
                ],
            )
    }

    fn source(&self) -> &str {
        self.source
    }
}

impl Display for MemRequestWordMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{{ op_label: {}, ctx: {}, addr: {}, clk: {}, word: {:?} }}",
            self.op_label, self.ctx, self.addr, self.clk, self.word
        )
    }
}

// MEMORY REQUEST ELEMENT MESSAGE
// ===============================================================================================

pub struct MemRequestElementMessage {
    pub op_label: Felt,
    pub ctx: Felt,
    pub addr: Felt,
    pub clk: Felt,
    pub element: Felt,
}

impl<E> BusMessage<E> for MemRequestElementMessage
where
    E: FieldElement<BaseField = Felt>,
{
    fn value(&self, alphas: &[E]) -> E {
        alphas[0]
            + build_value(
                &alphas[1..6],
                [self.op_label, self.ctx, self.addr, self.clk, self.element],
            )
    }

    fn source(&self) -> &str {
        "memory element"
    }
}

impl Display for MemRequestElementMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{{ op_label: {}, ctx: {}, addr: {}, clk: {}, element: {} }}",
            self.op_label, self.ctx, self.addr, self.clk, self.element
        )
    }
}
