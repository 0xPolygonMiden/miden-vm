use super::{
    ChipletsBus, Felt, Kernel, KernelProcLookup, KernelRom, TraceFragment, Word, ONE, TRACE_WIDTH,
    ZERO,
};
use crate::chiplets::{
    aux_trace::{ChipletLookup, ChipletsBusRow},
    ChipletsVTableTraceBuilder,
};
use vm_core::utils::collections::Vec;

// CONSTANTS
// ================================================================================================

const PROC1_HASH: Word = [ONE, ZERO, ONE, ZERO];
const PROC2_HASH: Word = [ONE, ONE, ONE, ONE];

// TESTS
// ================================================================================================

#[test]
fn kernel_rom_empty() {
    let kernel = Kernel::default();
    let rom = KernelRom::new(kernel);
    assert_eq!(0, rom.trace_len());

    // generate trace
    let (_, _, virtual_table) = build_trace(rom, 0);

    // make sure the chiplets table includes no kernel procedures
    verify_proc_table(&virtual_table, &[]);
}

#[test]
fn kernel_rom_invalid_access() {
    let kernel = build_kernel();
    let mut rom = KernelRom::new(kernel);

    // accessing procedure which is in the kernel should be fine
    assert!(rom.access_proc(PROC1_HASH.into()).is_ok());

    // accessing procedure which is not in the kernel should return an error
    assert!(rom.access_proc([ZERO, ONE, ZERO, ONE].into()).is_err());
}

#[test]
fn kernel_rom_no_access() {
    let kernel = build_kernel();
    let rom = KernelRom::new(kernel);

    let expected_trace_len = 2;
    assert_eq!(expected_trace_len, rom.trace_len());

    // generate trace
    let (trace, _, virtual_table) = build_trace(rom, expected_trace_len);

    // first row of the trace should correspond to the first procedure
    let row = 0;

    assert_eq!(trace[0][row], ZERO); // s0
    assert_eq!(trace[1][row], ZERO); // idx
    assert_eq!(trace[2][row], PROC1_HASH[0]);
    assert_eq!(trace[3][row], PROC1_HASH[1]);
    assert_eq!(trace[4][row], PROC1_HASH[2]);
    assert_eq!(trace[5][row], PROC1_HASH[3]);

    // second row of the trace should correspond to the second procedure
    let row = 1;

    assert_eq!(trace[0][row], ZERO); // s0
    assert_eq!(trace[1][row], ONE); // idx
    assert_eq!(trace[2][row], PROC2_HASH[0]);
    assert_eq!(trace[3][row], PROC2_HASH[1]);
    assert_eq!(trace[4][row], PROC2_HASH[2]);
    assert_eq!(trace[5][row], PROC2_HASH[3]);

    // make sure the chiplets table includes each kernel procedure exactly once
    verify_proc_table(&virtual_table, &[PROC1_HASH, PROC2_HASH]);
}

#[test]
fn kernel_rom_with_access() {
    let kernel = build_kernel();
    let mut rom = KernelRom::new(kernel);

    // generate 5 access: 3 for proc1 and 2 for proc2
    rom.access_proc(PROC1_HASH.into()).unwrap();
    rom.access_proc(PROC2_HASH.into()).unwrap();
    rom.access_proc(PROC1_HASH.into()).unwrap();
    rom.access_proc(PROC1_HASH.into()).unwrap();
    rom.access_proc(PROC2_HASH.into()).unwrap();

    let expected_trace_len = 5;
    assert_eq!(expected_trace_len, rom.trace_len());

    // generate trace
    let (trace, chiplets_bus, virtual_table) = build_trace(rom, expected_trace_len);

    // first 3 rows of the trace should correspond to the first procedure
    for row in 0..3 {
        assert_eq!(trace[0][row], ONE); // s0
        assert_eq!(trace[1][row], ZERO); // idx
        assert_eq!(trace[2][row], PROC1_HASH[0]);
        assert_eq!(trace[3][row], PROC1_HASH[1]);
        assert_eq!(trace[4][row], PROC1_HASH[2]);
        assert_eq!(trace[5][row], PROC1_HASH[3]);
    }

    // the remaining 2 rows of the trace should correspond to the second procedure
    for row in 3..5 {
        assert_eq!(trace[0][row], ONE); // s0
        assert_eq!(trace[1][row], ONE); // idx
        assert_eq!(trace[2][row], PROC2_HASH[0]);
        assert_eq!(trace[3][row], PROC2_HASH[1]);
        assert_eq!(trace[4][row], PROC2_HASH[2]);
        assert_eq!(trace[5][row], PROC2_HASH[3]);
    }

    // make sure the lookups were sent to the bus correctly from the kernel rom chiplet
    let proc1_lookup = KernelProcLookup::new(PROC1_HASH);
    let proc2_lookup = KernelProcLookup::new(PROC2_HASH);

    verify_bus(&chiplets_bus, 0, 0, &proc1_lookup);
    verify_bus(&chiplets_bus, 1, 1, &proc1_lookup);
    verify_bus(&chiplets_bus, 2, 2, &proc1_lookup);
    verify_bus(&chiplets_bus, 3, 3, &proc2_lookup);
    verify_bus(&chiplets_bus, 4, 4, &proc2_lookup);

    // make sure the chiplets table includes each kernel procedure exactly once
    verify_proc_table(&virtual_table, &[PROC1_HASH, PROC2_HASH]);
}

// HELPER FUNCTIONS
// ================================================================================================

/// Creates a kernel with two dummy procedures
fn build_kernel() -> Kernel {
    Kernel::new(&[PROC1_HASH.into(), PROC2_HASH.into()])
}

/// Builds a trace of the specified length and fills it with data from the provided KernelRom
/// instance.
fn build_trace(
    kernel_rom: KernelRom,
    num_rows: usize,
) -> (Vec<Vec<Felt>>, ChipletsBus, ChipletsVTableTraceBuilder) {
    let mut chiplets_bus = ChipletsBus::default();
    let mut virtual_table = ChipletsVTableTraceBuilder::default();
    let mut trace = (0..TRACE_WIDTH).map(|_| vec![ZERO; num_rows]).collect::<Vec<_>>();
    let mut fragment = TraceFragment::trace_to_fragment(&mut trace);
    kernel_rom.fill_trace(&mut fragment, &mut chiplets_bus, &mut virtual_table, 0);

    (trace, chiplets_bus, virtual_table)
}

/// Verifies that the chiplet bus received the specified KernelProcLookup response at `cycle` which
/// was added to the list of responses at `index`.
fn verify_bus(
    chiplets_bus: &ChipletsBus,
    index: usize,
    cycle: u32,
    proc_lookup: &KernelProcLookup,
) {
    let expected_lookup = ChipletLookup::KernelRom(*proc_lookup);
    let expected_hint = ChipletsBusRow::new(&[], Some(index as u32));

    let lookup = chiplets_bus.get_response_row(index);
    let hint = chiplets_bus.get_lookup_hint(cycle).unwrap();

    assert_eq!(expected_lookup, lookup);
    assert_eq!(&expected_hint, hint);
}

/// Verifies that the kernel procedure table contains every procedure in the kernel exactly once.
fn verify_proc_table(virtual_table: &ChipletsVTableTraceBuilder, proc_hashes: &[Word]) {
    // these tests are only for the kernel rom chiplet, so the virtual table should not be used by
    // other chiplets in these cases
    assert_eq!(virtual_table.rows().len(), proc_hashes.len());
    for (row, proc_hash) in virtual_table.rows().iter().zip(proc_hashes) {
        assert!(row.kernel_proc().is_some());
        if let Some(proc) = row.kernel_proc() {
            assert_eq!(proc.proc_hash(), *proc_hash);
        }
    }
}
