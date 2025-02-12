use core::fmt::{Display, Formatter, Result as FmtResult};

use miden_air::trace::chiplets::{
    hasher::{
        LINEAR_HASH_LABEL, MP_VERIFY_LABEL, MR_UPDATE_NEW_LABEL, MR_UPDATE_OLD_LABEL,
        RETURN_HASH_LABEL, RETURN_STATE_LABEL,
    },
    kernel_rom::KERNEL_PROC_LABEL,
};
use vm_core::{
    Felt, FieldElement, ONE, OPCODE_CALL, OPCODE_DYN, OPCODE_DYNCALL, OPCODE_JOIN, OPCODE_LOOP,
    OPCODE_SPLIT,
};

use super::build_value;
use crate::debug::BusMessage;

// CONTROL BLOCK MESSAGE
// ===============================================================================================

pub struct ControlBlockMessage {
    pub transition_label: Felt,
    pub addr_next: Felt,
    pub op_code: Felt,
    pub decoder_hasher_state: [Felt; 8],
}

impl<E> BusMessage<E> for ControlBlockMessage
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

impl Display for ControlBlockMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "transition_label: {}, addr_next: {}, op_code: {}, decoder_hasher_state: {:?}",
            self.transition_label, self.addr_next, self.op_code, self.decoder_hasher_state
        )
    }
}

// DYN BLOCK MESSAGE
// ===============================================================================================

pub struct DynBlockMessage {
    pub control_block_req: ControlBlockMessage,
    pub memory_req: MemRequestWordMessage,
}

impl<E> BusMessage<E> for DynBlockMessage
where
    E: FieldElement<BaseField = Felt>,
{
    fn value(&self, alphas: &[E]) -> E {
        self.control_block_req.value(alphas) * self.memory_req.value(alphas)
    }

    fn source(&self) -> &str {
        let op_code = self.control_block_req.op_code.as_int() as u8;
        match op_code {
            OPCODE_DYN => "dyn",
            OPCODE_DYNCALL => "dyncall",
            _ => panic!("unexpected opcode: {op_code}"),
        }
    }
}

impl Display for DynBlockMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "control block request: {} || memory_req: {}",
            self.control_block_req, self.memory_req
        )
    }
}

// SYSCALL BLOCK MESSAGE
// ===============================================================================================

pub struct SyscallBlockMessage {
    pub control_block_req: ControlBlockMessage,
    pub kernel_proc_digest: [Felt; 4],
}

impl<E> BusMessage<E> for SyscallBlockMessage
where
    E: FieldElement<BaseField = Felt>,
{
    fn value(&self, alphas: &[E]) -> E {
        let control_block_req_val = self.control_block_req.value(alphas);
        let kernel_rom_req_val = alphas[0]
            + alphas[1].mul_base(KERNEL_PROC_LABEL)
            + alphas[2].mul_base(self.kernel_proc_digest[0])
            + alphas[3].mul_base(self.kernel_proc_digest[1])
            + alphas[4].mul_base(self.kernel_proc_digest[2])
            + alphas[5].mul_base(self.kernel_proc_digest[3]);

        control_block_req_val * kernel_rom_req_val
    }

    fn source(&self) -> &str {
        "syscall"
    }
}

impl Display for SyscallBlockMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "control_block_req: {}, op_label: {}, state: {:?}",
            self.control_block_req, KERNEL_PROC_LABEL, self.kernel_proc_digest
        )
    }
}

// SPAN BLOCK MESSAGE
// ===============================================================================================

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
            "transition_label: {}, addr_next: {}, state: {:?}",
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
            "transition_label: {}, addr_next: {}, state: {:?}",
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
            "addr: {}, transition_label: {}, digest: {:?}",
            self.addr, self.transition_label, self.digest
        )
    }
}

// BITWISE REQUEST MESSAGE
// ===============================================================================================

pub struct BitwiseRequestMessage {
    pub op_label: Felt,
    pub a: Felt,
    pub b: Felt,
    pub z: Felt,
}

impl<E> BusMessage<E> for BitwiseRequestMessage
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

impl Display for BitwiseRequestMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "op_label: {}, a: {}, b: {}, z: {}", self.op_label, self.a, self.b, self.z)
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
        "memory word"
    }
}

impl Display for MemRequestWordMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "op_label: {}, ctx: {}, addr: {}, clk: {}, word: {:?}",
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
            "op_label: {}, ctx: {}, addr: {}, clk: {}, element: {}",
            self.op_label, self.ctx, self.addr, self.clk, self.element
        )
    }
}

// MSTREAM REQUEST MESSAGE
// ===============================================================================================

pub struct MstreamRequestMessage {
    pub mem_req_1: MemRequestWordMessage,
    pub mem_req_2: MemRequestWordMessage,
}

impl<E> BusMessage<E> for MstreamRequestMessage
where
    E: FieldElement<BaseField = Felt>,
{
    fn value(&self, alphas: &[E]) -> E {
        self.mem_req_1.value(alphas) * self.mem_req_2.value(alphas)
    }

    fn source(&self) -> &str {
        "mstream"
    }
}

impl Display for MstreamRequestMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "mem_req_1: {} || mem_req_2: {}", self.mem_req_1, self.mem_req_2)
    }
}

// PIPE REQUEST MESSAGE
// ===============================================================================================

pub struct PipeRequestMessage {
    pub mem_req_1: MemRequestWordMessage,
    pub mem_req_2: MemRequestWordMessage,
}

impl<E> BusMessage<E> for PipeRequestMessage
where
    E: FieldElement<BaseField = Felt>,
{
    fn value(&self, alphas: &[E]) -> E {
        self.mem_req_1.value(alphas) * self.mem_req_2.value(alphas)
    }

    fn source(&self) -> &str {
        "pipe"
    }
}

impl Display for PipeRequestMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "mem_req_1: {} || mem_req_2: {}", self.mem_req_1, self.mem_req_2)
    }
}

// RCOMB BASE REQUEST MESSAGE
// ===============================================================================================

pub struct RcombBaseRequestMessage {
    pub mem_req_1: MemRequestWordMessage,
    pub mem_req_2: MemRequestWordMessage,
}

impl<E> BusMessage<E> for RcombBaseRequestMessage
where
    E: FieldElement<BaseField = Felt>,
{
    fn value(&self, alphas: &[E]) -> E {
        self.mem_req_1.value(alphas) * self.mem_req_2.value(alphas)
    }

    fn source(&self) -> &str {
        "rcombbase"
    }
}

impl Display for RcombBaseRequestMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "mem_req_1: {} || mem_req_2: {}", self.mem_req_1, self.mem_req_2)
    }
}

// HPERM REQUEST MESSAGE
// ===============================================================================================

pub struct HpermRequestMessage {
    pub helper_0: Felt,
    pub s0_s12_cur: [Felt; 12],
    pub s0_s12_nxt: [Felt; 12],
}

impl<E> BusMessage<E> for HpermRequestMessage
where
    E: FieldElement<BaseField = Felt>,
{
    fn value(&self, alphas: &[E]) -> E {
        let v_input_req = {
            let op_label = LINEAR_HASH_LABEL + 16;

            let sum_input = alphas[4..16]
                .iter()
                .rev()
                .enumerate()
                .fold(E::ZERO, |acc, (i, x)| acc + x.mul_base(self.s0_s12_cur[i]));

            alphas[0]
                + alphas[1].mul_base(Felt::from(op_label))
                + alphas[2].mul_base(self.helper_0)
                + sum_input
        };

        let v_output_req = {
            let op_label = RETURN_STATE_LABEL + 32;

            let sum_output = alphas[4..16]
                .iter()
                .rev()
                .enumerate()
                .fold(E::ZERO, |acc, (i, x)| acc + x.mul_base(self.s0_s12_nxt[i]));
            alphas[0]
                + alphas[1].mul_base(Felt::from(op_label))
                + alphas[2].mul_base(self.helper_0 + Felt::new(7))
                + sum_output
        };

        v_input_req * v_output_req
    }

    fn source(&self) -> &str {
        "hperm"
    }
}

impl Display for HpermRequestMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "helper_0: {}, s0_s12_cur: {:?}, s0_s12_nxt: {:?}",
            self.helper_0, self.s0_s12_cur, self.s0_s12_nxt
        )
    }
}

// MPVERIFY REQUEST MESSAGE
// ===============================================================================================

pub struct MpverifyRequestMessage {
    pub helper_0: Felt,
    pub s0_s3: [Felt; 4],
    pub s4: Felt,
    pub s5: Felt,
    pub s6_s9: [Felt; 4],
}

impl<E> BusMessage<E> for MpverifyRequestMessage
where
    E: FieldElement<BaseField = Felt>,
{
    fn value(&self, alphas: &[E]) -> E {
        let op_label = MP_VERIFY_LABEL + 16;

        let v_input = {
            let sum_input = alphas[8..12]
                .iter()
                .rev()
                .enumerate()
                .fold(E::ZERO, |acc, (i, x)| acc + x.mul_base(self.s0_s3[i]));

            alphas[0]
                + alphas[1].mul_base(Felt::from(op_label))
                + alphas[2].mul_base(self.helper_0)
                + alphas[3].mul_base(self.s5)
                + sum_input
        };

        let v_output = {
            let op_label = RETURN_HASH_LABEL + 32;

            let sum_output = alphas[8..12]
                .iter()
                .rev()
                .enumerate()
                .fold(E::ZERO, |acc, (i, x)| acc + x.mul_base(self.s6_s9[i]));
            alphas[0]
                + alphas[1].mul_base(Felt::from(op_label))
                + alphas[2].mul_base(self.helper_0 + self.s4.mul_small(8) - ONE)
                + sum_output
        };

        v_input * v_output
    }

    fn source(&self) -> &str {
        "mpverify"
    }
}

impl Display for MpverifyRequestMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "helper_0: {}, s0_s3: {:?}, s4: {}, s5: {}, s6_s9: {:?}",
            self.helper_0, self.s0_s3, self.s4, self.s5, self.s6_s9
        )
    }
}

// MRUPDATE REQUEST MESSAGE
// ===============================================================================================

pub struct MrupdateRequestMessage {
    pub helper_0: Felt,
    pub s0_s3: [Felt; 4],
    pub s0_s3_nxt: [Felt; 4],
    pub s4: Felt,
    pub s5: Felt,
    pub s6_s9: [Felt; 4],
    pub s10_s13: [Felt; 4],
}

impl<E> BusMessage<E> for MrupdateRequestMessage
where
    E: FieldElement<BaseField = Felt>,
{
    fn value(&self, alphas: &[E]) -> E {
        let v_input_old = {
            let op_label = MR_UPDATE_OLD_LABEL + 16;

            let sum_input = alphas[8..12]
                .iter()
                .rev()
                .enumerate()
                .fold(E::ZERO, |acc, (i, x)| acc + x.mul_base(self.s0_s3[i]));
            alphas[0]
                + alphas[1].mul_base(Felt::from(op_label))
                + alphas[2].mul_base(self.helper_0)
                + alphas[3].mul_base(self.s5)
                + sum_input
        };

        let v_output_old = {
            let op_label = RETURN_HASH_LABEL + 32;

            let sum_output = alphas[8..12]
                .iter()
                .rev()
                .enumerate()
                .fold(E::ZERO, |acc, (i, x)| acc + x.mul_base(self.s6_s9[i]));
            alphas[0]
                + alphas[1].mul_base(Felt::from(op_label))
                + alphas[2].mul_base(self.helper_0 + self.s4.mul_small(8) - ONE)
                + sum_output
        };

        let v_input_new = {
            let op_label = MR_UPDATE_NEW_LABEL + 16;
            let sum_input = alphas[8..12]
                .iter()
                .rev()
                .enumerate()
                .fold(E::ZERO, |acc, (i, x)| acc + x.mul_base(self.s10_s13[i]));
            alphas[0]
                + alphas[1].mul_base(Felt::from(op_label))
                + alphas[2].mul_base(self.helper_0 + self.s4.mul_small(8))
                + alphas[3].mul_base(self.s5)
                + sum_input
        };

        let v_output_new = {
            let op_label = RETURN_HASH_LABEL + 32;

            let sum_output = alphas[8..12]
                .iter()
                .rev()
                .enumerate()
                .fold(E::ZERO, |acc, (i, x)| acc + x.mul_base(self.s0_s3_nxt[i]));
            alphas[0]
                + alphas[1].mul_base(Felt::from(op_label))
                + alphas[2].mul_base(self.helper_0 + self.s4.mul_small(16) - ONE)
                + sum_output
        };

        v_input_old * v_output_old * v_input_new * v_output_new
    }

    fn source(&self) -> &str {
        "mrupdate"
    }
}

impl Display for MrupdateRequestMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "helper_0: {}, s0_s3: {:?}, s0_s3_nxt: {:?}, s4: {}, s5: {}, s6_s9: {:?}, s10_s13: {:?}",
            self.helper_0,
            self.s0_s3,
            self.s0_s3_nxt,
            self.s4,
            self.s5,
            self.s6_s9,
            self.s10_s13,
        )
    }
}
