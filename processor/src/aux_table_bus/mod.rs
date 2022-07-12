use super::{BTreeMap, Felt, FieldElement, StarkField, Vec, Word};
use crate::{memory::MemoryLookup, trace::LookupTableRow};

mod aux_trace;
pub use aux_trace::AuxTraceBuilder;

// AUXILIARY TABLE BUS
// ================================================================================================

/// The Auxiliary Table Bus tracks data requested from or provided by co-processors in the
/// Auxiliary Table. It processes lookup requests from the stack and response data from the
/// co-processors in the Auxiliary Table.
///
/// For correct execution, the lookup data used by the stack for each co-processor must be a
/// permutation of the lookups executed by that co-processor so that they cancel out. This is
/// ensured by the `b_aux` bus column. When the `b_aux` column is built, requests from the stack
/// must be divided out and lookup results provided by the co-processors must be multiplied in. To
/// ensure that all lookups are attributed to the correct co-processor and operation, a unique
/// co-processor operation selector must be included in the lookup row value when it is computed.
///
/// TODO: document the AuxTableBus components and their types.

#[derive(Default)]
pub struct AuxTableBus {
    lookup_hints: BTreeMap<usize, AuxTableLookup>,
    request_rows: Vec<AuxTableLookupRow>,
    response_rows: Vec<AuxTableLookupRow>,
}

impl AuxTableBus {
    // LOOKUP MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Requests a memory access with the specified data. When `old_word` and `new_word` are the
    /// same, this is a read request. When they are different, it's a write request. The memory
    /// value is requested at cycle `clk`. This is expected to be called by operation executors.
    pub fn request_memory_operation(
        &mut self,
        addr: Felt,
        clk: usize,
        old_word: Word,
        new_word: Word,
    ) {
        // all requests are sent from the stack before responses are provided (during AuxTable trace
        // finalization). requests are guaranteed not to share cycles with other requests, since
        // only one operation will be executed at a time.
        let request_idx = self.request_rows.len();
        self.lookup_hints
            .insert(clk, AuxTableLookup::Request(request_idx));

        let memory_lookup = MemoryLookup::new(addr, clk as u64, old_word, new_word);
        self.request_rows
            .push(AuxTableLookupRow::Memory(memory_lookup));
    }

    /// Provides the data of a memory read or write contained in the [Memory] table.  When
    /// `old_word` and `new_word` are the same, this is a read request. When they are different,
    /// it's a write  request. The memory value is provided at cycle `response_cycle`, which is the
    /// row of the execution trace that contains this Memory row.
    pub fn provide_memory_operation(
        &mut self,
        addr: Felt,
        clk: Felt,
        old_word: Word,
        new_word: Word,
        response_cycle: usize,
    ) {
        // results are guaranteed not to share cycles with other results, but they might share
        // a cycle with a request which has already been sent.
        let response_idx = self.response_rows.len();
        self.lookup_hints
            .entry(response_cycle)
            .and_modify(|lookup| {
                if let AuxTableLookup::Request(request_idx) = *lookup {
                    *lookup = AuxTableLookup::RequestAndResponse((request_idx, response_idx));
                }
            })
            .or_insert_with(|| AuxTableLookup::Response(response_idx));

        let memory_lookup = MemoryLookup::new(addr, clk.as_int(), old_word, new_word);
        self.response_rows
            .push(AuxTableLookupRow::Memory(memory_lookup));
    }

    // AUX TRACE BUILDER GENERATION
    // --------------------------------------------------------------------------------------------

    /// Converts this [AuxTableBus] into an auxiliary trace builder which can be used to construct
    /// the auxiliary trace column describing the [AuxTable] lookups at every cycle.
    pub fn into_aux_builder(self) -> AuxTraceBuilder {
        let lookup_hints = self.lookup_hints.into_iter().collect();

        AuxTraceBuilder {
            lookup_hints,
            request_rows: self.request_rows,
            response_rows: self.response_rows,
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns an option with the lookup hint for the specified cycle.
    #[cfg(test)]
    pub(super) fn get_lookup_hint(&self, cycle: usize) -> Option<&AuxTableLookup> {
        self.lookup_hints.get(&cycle)
    }

    /// Returns the ith lookup response provided by the AuxTable co-processors.
    #[cfg(test)]
    pub(super) fn get_response_row(&self, i: usize) -> AuxTableLookupRow {
        self.response_rows[i]
    }
}

// AUXILIARY TABLE LOOKUPS
// ================================================================================================

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(super) enum AuxTableLookup {
    Request(usize),
    Response(usize),
    RequestAndResponse((usize, usize)),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub(super) enum AuxTableLookupRow {
    Hasher(HasherLookupRow),
    Bitwise(BitwiseLookupRow),
    Memory(MemoryLookup),
}

impl LookupTableRow for AuxTableLookupRow {
    fn to_value<E: FieldElement<BaseField = Felt>>(&self, alphas: &[E]) -> E {
        match self {
            AuxTableLookupRow::Hasher(row) => row.to_value(alphas),
            AuxTableLookupRow::Bitwise(row) => row.to_value(alphas),
            AuxTableLookupRow::Memory(row) => row.to_value(alphas),
        }
    }
}

// HASH PROCESSOR LOOKUPS
// ================================================================================================

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(super) struct HasherLookupRow {}

impl LookupTableRow for HasherLookupRow {
    /// Reduces this row to a single field element in the field specified by E. This requires
    /// at least 12 alpha values.
    fn to_value<E: FieldElement<BaseField = Felt>>(&self, _alphas: &[E]) -> E {
        unimplemented!()
    }
}

// BITWISE PROCESSOR LOOKUPS
// ================================================================================================

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(super) struct BitwiseLookupRow {}

impl LookupTableRow for BitwiseLookupRow {
    /// Reduces this row to a single field element in the field specified by E. This requires
    /// at least 12 alpha values.
    fn to_value<E: FieldElement<BaseField = Felt>>(&self, _alphas: &[E]) -> E {
        unimplemented!()
    }
}
