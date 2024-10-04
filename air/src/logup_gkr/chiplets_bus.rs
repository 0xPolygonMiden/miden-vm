use vm_core::{ExtensionOf, FieldElement};

use super::{inner_product, LogUpOpFlags};
use crate::{
    decoder::{DECODER_HASHER_STATE_OFFSET, DECODER_USER_OP_HELPERS_OFFSET},
    trace::{
        chiplets::{
            kernel_rom::KERNEL_PROC_LABEL_RAW,
            memory::{MEMORY_READ_LABEL, MEMORY_WRITE_LABEL},
            BITWISE_A_COL_IDX, BITWISE_B_COL_IDX, BITWISE_OUTPUT_COL_IDX, MEMORY_ADDR_COL_IDX,
            MEMORY_CLK_COL_IDX, MEMORY_CTX_COL_IDX, MEMORY_V_COL_RANGE,
        },
        stack::STACK_TOP_OFFSET,
    },
    CHIPLETS_OFFSET, CLK_COL_IDX, CTX_COL_IDX, STACK_TRACE_OFFSET,
};

#[inline(always)]
pub fn bus<F, E>(
    query_current: &[F],
    query_next: &[F],
    bitwise_periodic_values: &[F],
    op_flags: &LogUpOpFlags<F>,
    alphas: &[E],
    numerator: &mut [E],
    denominator: &mut [E],
) where
    F: FieldElement,
    E: FieldElement + ExtensionOf<F>,
{
    let bitwise_reponse =
        get_bitwise_chiplet_response(query_current, bitwise_periodic_values, alphas);
    let [u32xor_request, u32and_request] =
        get_u32andxor_requests(query_current, query_next, op_flags, alphas);
    let kernel_chiplet_response = get_kernel_chiplet_response(query_current, alphas);
    let syscall_kernel_request = get_syscall_kernel_request(query_current, op_flags, alphas);
    let memory_chiplet_response = get_memory_chiplet_response(query_current, alphas);
    let [mload_req, mstore_req] =
        get_mload_mstore_requests(query_current, query_next, op_flags, alphas);
    let [mloadw_req, mstorew_req] =
        get_mloadw_mstorew_requests(query_current, query_next, op_flags, alphas);
    let [pipe_req1, pipe_req2] =
        get_pipe_memory_requests(query_current, query_next, op_flags, alphas);
    let [mstream_req1, mstream_req2] =
        get_mstream_memory_requests(query_current, query_next, op_flags, alphas);
    let [rcombbase_req1, rcombbase_req2] =
        get_rcombbase_memory_requests(query_current, op_flags, alphas);

    // bitwise chiplet
    (numerator[0], denominator[0]) = bitwise_reponse;
    (numerator[1], denominator[1]) = u32xor_request;
    (numerator[2], denominator[2]) = u32and_request;

    // kernel chiplet
    (numerator[3], denominator[3]) = kernel_chiplet_response;
    (numerator[4], denominator[4]) = syscall_kernel_request;

    // memory chiplet
    (numerator[5], denominator[5]) = memory_chiplet_response;
    (numerator[6], denominator[6]) = mload_req;
    (numerator[7], denominator[7]) = mstore_req;
    (numerator[8], denominator[8]) = mloadw_req;
    (numerator[9], denominator[9]) = mstorew_req;
    (numerator[10], denominator[10]) = pipe_req1;
    (numerator[11], denominator[11]) = pipe_req2;
    (numerator[12], denominator[12]) = mstream_req1;
    (numerator[13], denominator[13]) = mstream_req2;
    (numerator[14], denominator[14]) = rcombbase_req1;
    (numerator[15], denominator[15]) = rcombbase_req2;
}

// RESPONSES
// ===============================================================================================

#[inline(always)]
fn get_bitwise_chiplet_response<F, E>(
    query_current: &[F],
    bitwise_periodic_values: &[F],
    alphas: &[E],
) -> (E, E)
where
    F: FieldElement,
    E: FieldElement + ExtensionOf<F>,
{
    let numerator = {
        let f_is_bitwise_chiplet: E = {
            let bitwise_selec0 = query_current[CHIPLETS_OFFSET];
            let bitwise_selec1 = F::ONE - query_current[CHIPLETS_OFFSET + 1];

            (bitwise_selec0 * bitwise_selec1).into()
        };
        let is_last_periodic_row: E = (F::ONE - bitwise_periodic_values[1]).into();

        f_is_bitwise_chiplet * is_last_periodic_row
    };
    let denominator = {
        let is_xor = query_current[CHIPLETS_OFFSET + 2];
        let op_label_is_xor = get_op_label(F::ONE, F::ZERO, is_xor, F::ZERO);
        let a = query_current[BITWISE_A_COL_IDX];
        let b = query_current[BITWISE_B_COL_IDX];
        let z = query_current[BITWISE_OUTPUT_COL_IDX];

        alphas[0] + inner_product(&alphas[1..5], &[op_label_is_xor, a, b, z])
    };

    (numerator, denominator)
}

#[inline(always)]
fn get_kernel_chiplet_response<F, E>(query_current: &[F], alphas: &[E]) -> (E, E)
where
    F: FieldElement,
    E: FieldElement + ExtensionOf<F>,
{
    // numerator
    let numerator = {
        let is_kernel_chiplet = {
            let selector0 = query_current[CHIPLETS_OFFSET];
            let selector1 = query_current[CHIPLETS_OFFSET + 1];
            let selector2 = query_current[CHIPLETS_OFFSET + 2];
            let selector3 = query_current[CHIPLETS_OFFSET + 3];

            selector0 * selector1 * selector2 * (F::ONE - selector3)
        };

        let include_row_in_bus = query_current[CHIPLETS_OFFSET + 4];

        E::from(is_kernel_chiplet * include_row_in_bus)
    };

    // denominator
    let denominator = {
        let root0 = query_current[CHIPLETS_OFFSET + 6];
        let root1 = query_current[CHIPLETS_OFFSET + 7];
        let root2 = query_current[CHIPLETS_OFFSET + 8];
        let root3 = query_current[CHIPLETS_OFFSET + 9];

        alphas[0]
            + inner_product(
                &alphas[1..6],
                &[KERNEL_PROC_LABEL_RAW.into(), root0, root1, root2, root3],
            )
    };

    (numerator, denominator)
}

#[inline(always)]
fn get_memory_chiplet_response<F, E>(query_current: &[F], alphas: &[E]) -> (E, E)
where
    F: FieldElement,
    E: FieldElement + ExtensionOf<F>,
{
    let is_memory_chiplet = {
        let selector0 = query_current[CHIPLETS_OFFSET];
        let selector1 = query_current[CHIPLETS_OFFSET + 1];
        let selector2 = query_current[CHIPLETS_OFFSET + 2];

        selector0 * selector1 * (F::ONE - selector2)
    };

    let denominator = {
        let op_label = {
            let is_read = query_current[CHIPLETS_OFFSET + 3];
            get_op_label(F::ONE, F::ONE, F::ZERO, is_read)
        };
        let ctx = query_current[MEMORY_CTX_COL_IDX];
        let clk = query_current[MEMORY_CLK_COL_IDX];
        let mem_addr = query_current[MEMORY_ADDR_COL_IDX];
        let mem_value = &query_current[MEMORY_V_COL_RANGE];

        compute_memory_message(alphas, op_label, ctx, mem_addr, clk, mem_value)
    };

    (is_memory_chiplet.into(), denominator)
}

// REQUESTS
// ===============================================================================================

#[inline(always)]
fn get_u32andxor_requests<F, E>(
    query_current: &[F],
    query_next: &[F],
    op_flags: &LogUpOpFlags<F>,
    alphas: &[E],
) -> [(E, E); 2]
where
    F: FieldElement,
    E: FieldElement + ExtensionOf<F>,
{
    // numerators
    let u32xor_numerator = op_flags.f_u32_xor().into();
    let u32and_numerator = op_flags.f_u32_and().into();

    // denominators
    let op_label_is_xor = get_op_label(F::ONE, F::ZERO, F::ONE, F::ZERO);
    let op_label_is_and = get_op_label(F::ONE, F::ZERO, F::ZERO, F::ZERO);
    let a = query_current[STACK_TRACE_OFFSET + STACK_TOP_OFFSET + 1];
    let b = query_current[STACK_TRACE_OFFSET + STACK_TOP_OFFSET];
    let z = query_next[STACK_TRACE_OFFSET + STACK_TOP_OFFSET];

    let u32xor_denominator =
        -(alphas[0] + inner_product(&alphas[1..5], &[op_label_is_xor, a, b, z]));
    let u32and_denominator =
        -(alphas[0] + inner_product(&alphas[1..5], &[op_label_is_and, a, b, z]));

    [(u32xor_numerator, u32xor_denominator), (u32and_numerator, u32and_denominator)]
}

#[inline(always)]
fn get_syscall_kernel_request<F, E>(
    query_current: &[F],
    op_flags: &LogUpOpFlags<F>,
    alphas: &[E],
) -> (E, E)
where
    F: FieldElement,
    E: FieldElement + ExtensionOf<F>,
{
    let numerator: E = op_flags.f_syscall().into();

    let denominator = {
        let op_label: F = KERNEL_PROC_LABEL_RAW.into();
        let root0 = query_current[DECODER_HASHER_STATE_OFFSET];
        let root1 = query_current[DECODER_HASHER_STATE_OFFSET + 1];
        let root2 = query_current[DECODER_HASHER_STATE_OFFSET + 2];
        let root3 = query_current[DECODER_HASHER_STATE_OFFSET + 3];

        -(alphas[0] + inner_product(&alphas[1..6], &[op_label, root0, root1, root2, root3]))
    };

    (numerator, denominator)
}

/// Returns the memory request for `MLOAD` and `MSTORE`.
#[inline(always)]
fn get_mload_mstore_requests<F, E>(
    query_current: &[F],
    query_next: &[F],
    op_flags: &LogUpOpFlags<F>,
    alphas: &[E],
) -> [(E, E); 2]
where
    F: FieldElement,
    E: FieldElement + ExtensionOf<F>,
{
    // the value read or written
    let mem_value = [
        query_next[STACK_TRACE_OFFSET + STACK_TOP_OFFSET],
        query_current[DECODER_USER_OP_HELPERS_OFFSET + 2],
        query_current[DECODER_USER_OP_HELPERS_OFFSET + 1],
        query_current[DECODER_USER_OP_HELPERS_OFFSET],
    ];
    let mem_addr = query_current[STACK_TRACE_OFFSET + STACK_TOP_OFFSET];
    let ctx = query_current[CTX_COL_IDX];
    let clk = query_current[CLK_COL_IDX];

    let mload_req = (
        op_flags.f_mload().into(),
        -compute_memory_message(alphas, MEMORY_READ_LABEL.into(), ctx, mem_addr, clk, &mem_value),
    );

    let mstore_req = (
        op_flags.f_mstore().into(),
        -compute_memory_message(alphas, MEMORY_WRITE_LABEL.into(), ctx, mem_addr, clk, &mem_value),
    );

    [mload_req, mstore_req]
}

/// Returns the memory request for `MLOADW` and `MSTOREW`.
#[inline(always)]
fn get_mloadw_mstorew_requests<F, E>(
    query_current: &[F],
    query_next: &[F],
    op_flags: &LogUpOpFlags<F>,
    alphas: &[E],
) -> [(E, E); 2]
where
    F: FieldElement,
    E: FieldElement + ExtensionOf<F>,
{
    // the value read or written
    let mem_value = [
        query_next[STACK_TRACE_OFFSET + STACK_TOP_OFFSET + 3],
        query_next[STACK_TRACE_OFFSET + STACK_TOP_OFFSET + 2],
        query_next[STACK_TRACE_OFFSET + STACK_TOP_OFFSET + 1],
        query_next[STACK_TRACE_OFFSET + STACK_TOP_OFFSET],
    ];
    let mem_addr = query_current[STACK_TRACE_OFFSET + STACK_TOP_OFFSET];
    let ctx = query_current[CTX_COL_IDX];
    let clk = query_current[CLK_COL_IDX];

    let mloadw_req = (
        op_flags.f_mloadw().into(),
        -compute_memory_message(alphas, MEMORY_READ_LABEL.into(), ctx, mem_addr, clk, &mem_value),
    );

    let mstorew_req = (
        op_flags.f_mstorew().into(),
        -compute_memory_message(alphas, MEMORY_WRITE_LABEL.into(), ctx, mem_addr, clk, &mem_value),
    );

    [mloadw_req, mstorew_req]
}

/// Returns the memory requests associated with the `PIPE` operation.
#[inline(always)]
fn get_pipe_memory_requests<F, E>(
    query_current: &[F],
    query_next: &[F],
    op_flags: &LogUpOpFlags<F>,
    alphas: &[E],
) -> [(E, E); 2]
where
    F: FieldElement,
    E: FieldElement + ExtensionOf<F>,
{
    let f_pipe_flag: E = op_flags.f_pipe().into();
    let op_label: F = MEMORY_WRITE_LABEL.into();
    let ctx = query_current[CTX_COL_IDX];
    let clk = query_current[CLK_COL_IDX];

    let req1 = {
        let mem_value = [
            query_next[STACK_TRACE_OFFSET + STACK_TOP_OFFSET + 7],
            query_next[STACK_TRACE_OFFSET + STACK_TOP_OFFSET + 6],
            query_next[STACK_TRACE_OFFSET + STACK_TOP_OFFSET + 5],
            query_next[STACK_TRACE_OFFSET + STACK_TOP_OFFSET + 4],
        ];
        let mem_addr = query_current[STACK_TRACE_OFFSET + STACK_TOP_OFFSET + 12];

        (
            f_pipe_flag,
            -compute_memory_message(alphas, op_label, ctx, mem_addr, clk, &mem_value),
        )
    };
    let req2 = {
        let mem_value = [
            query_next[STACK_TRACE_OFFSET + STACK_TOP_OFFSET + 3],
            query_next[STACK_TRACE_OFFSET + STACK_TOP_OFFSET + 2],
            query_next[STACK_TRACE_OFFSET + STACK_TOP_OFFSET + 1],
            query_next[STACK_TRACE_OFFSET + STACK_TOP_OFFSET],
        ];
        let mem_addr = query_current[STACK_TRACE_OFFSET + STACK_TOP_OFFSET + 12] + F::ONE;

        (
            f_pipe_flag,
            -compute_memory_message(alphas, op_label, ctx, mem_addr, clk, &mem_value),
        )
    };

    [req1, req2]
}

/// Returns the memory requests associated with the `MSTREAM` operation.
#[inline(always)]
fn get_mstream_memory_requests<F, E>(
    query_current: &[F],
    query_next: &[F],
    op_flags: &LogUpOpFlags<F>,
    alphas: &[E],
) -> [(E, E); 2]
where
    F: FieldElement,
    E: FieldElement + ExtensionOf<F>,
{
    let f_mstream_flag: E = op_flags.f_mstream().into();
    let op_label: F = MEMORY_READ_LABEL.into();
    let ctx = query_current[CTX_COL_IDX];
    let clk = query_current[CLK_COL_IDX];

    let req1 = {
        let mem_value = [
            query_next[STACK_TRACE_OFFSET + STACK_TOP_OFFSET + 7],
            query_next[STACK_TRACE_OFFSET + STACK_TOP_OFFSET + 6],
            query_next[STACK_TRACE_OFFSET + STACK_TOP_OFFSET + 5],
            query_next[STACK_TRACE_OFFSET + STACK_TOP_OFFSET + 4],
        ];
        let mem_addr = query_current[STACK_TRACE_OFFSET + STACK_TOP_OFFSET + 12];

        (
            f_mstream_flag,
            -compute_memory_message(alphas, op_label, ctx, mem_addr, clk, &mem_value),
        )
    };
    let req2 = {
        let mem_value = [
            query_next[STACK_TRACE_OFFSET + STACK_TOP_OFFSET + 3],
            query_next[STACK_TRACE_OFFSET + STACK_TOP_OFFSET + 2],
            query_next[STACK_TRACE_OFFSET + STACK_TOP_OFFSET + 1],
            query_next[STACK_TRACE_OFFSET + STACK_TOP_OFFSET],
        ];
        let mem_addr = query_current[STACK_TRACE_OFFSET + STACK_TOP_OFFSET + 12] + F::ONE;

        (
            f_mstream_flag,
            -compute_memory_message(alphas, op_label, ctx, mem_addr, clk, &mem_value),
        )
    };

    [req1, req2]
}

/// Returns the memory requests associated with the `RCOMBBASE` operation.
#[inline(always)]
fn get_rcombbase_memory_requests<F, E>(
    query_current: &[F],
    op_flags: &LogUpOpFlags<F>,
    alphas: &[E],
) -> [(E, E); 2]
where
    F: FieldElement,
    E: FieldElement + ExtensionOf<F>,
{
    let f_rcombbase_flag: E = op_flags.f_rcombbase().into();
    let op_label: F = MEMORY_READ_LABEL.into();
    let ctx = query_current[CTX_COL_IDX];
    let clk = query_current[CLK_COL_IDX];

    let req1 = {
        let mem_value = {
            let tz0 = query_current[DECODER_USER_OP_HELPERS_OFFSET];
            let tz1 = query_current[DECODER_USER_OP_HELPERS_OFFSET + 1];
            let tzg0 = query_current[DECODER_USER_OP_HELPERS_OFFSET + 2];
            let tzg1 = query_current[DECODER_USER_OP_HELPERS_OFFSET + 3];
            [tz0, tz1, tzg0, tzg1]
        };
        let mem_addr = query_current[STACK_TRACE_OFFSET + STACK_TOP_OFFSET + 13];

        (
            f_rcombbase_flag,
            -compute_memory_message(alphas, op_label, ctx, mem_addr, clk, &mem_value),
        )
    };

    let req2 = {
        let mem_value = {
            let a0 = query_current[DECODER_USER_OP_HELPERS_OFFSET + 4];
            let a1 = query_current[DECODER_USER_OP_HELPERS_OFFSET + 5];

            [a0, a1, F::ZERO, F::ZERO]
        };
        let mem_addr = query_current[STACK_TRACE_OFFSET + STACK_TOP_OFFSET + 14];

        (
            f_rcombbase_flag,
            -compute_memory_message(alphas, op_label, ctx, mem_addr, clk, &mem_value),
        )
    };

    [req1, req2]
}

// HELPERS
// ==============================================================================================

/// Computes a memory message (request or response) given randomness `alphas`.
#[inline(always)]
fn compute_memory_message<F, E>(
    alphas: &[E],
    op_label: F,
    ctx: F,
    mem_addr: F,
    clk: F,
    mem_value: &[F],
) -> E
where
    F: FieldElement,
    E: FieldElement + ExtensionOf<F>,
{
    alphas[0]
        + inner_product(
            alphas,
            &[
                op_label,
                ctx,
                mem_addr,
                clk,
                mem_value[0],
                mem_value[1],
                mem_value[2],
                mem_value[3],
            ],
        )
}

/// Returns the operation unique label.
fn get_op_label<F>(s0: F, s1: F, s2: F, s3: F) -> F
where
    F: FieldElement,
{
    s3 * (1_u32 << 3).into() + s2 * (1_u32 << 2).into() + s1 * 2_u32.into() + s0 + F::ONE
}
