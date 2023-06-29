use super::{
    super::{hasher::HasherLookup, BitwiseLookup, KernelProcLookup, MemoryLookup},
    BTreeMap, BusTraceBuilder, ColMatrix, Felt, FieldElement, LookupTableRow, Vec,
};

// CHIPLETS BUS
// ================================================================================================

/// The Chiplets bus tracks data requested from or provided by chiplets in the Chiplets module. It
/// processes lookup requests from the stack & decoder and response data from the chiplets.
///
/// For correct execution, the lookup data used by the stack for each chiplet must be a permutation
/// of the lookups executed by that chiplet so that they cancel out. This is ensured by the `b_chip`
/// bus column. When the `b_chip` column is built, requests from the stack must be divided out and
/// lookup results provided by the chiplets must be multiplied in. To ensure that all lookups are
/// attributed to the correct chiplet and operation, a unique chiplet operation label must be
/// included in the lookup row value when it is computed.

#[derive(Default)]
pub struct ChipletsBus {
    lookup_hints: BTreeMap<u32, ChipletsBusRow>,
    requests: Vec<ChipletLookup>,
    responses: Vec<ChipletLookup>,
    // TODO: remove queued requests by refactoring the hasher/decoder interactions so that the
    // lookups are built as they are requested. This will be made easier by removing state info from
    // the HasherLookup struct. Primarily it will require a refactor of `hash_span_block`,
    // `start_span_block`, `respan`, and `end_span_block`.
    queued_requests: Vec<HasherLookup>,
}

impl ChipletsBus {
    // LOOKUP MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Requests lookups for a single operation at the specified cycle. A Hasher operation request
    /// can contain one or more lookups, while Bitwise and Memory requests will only contain a
    /// single lookup.
    fn request_lookups(&mut self, request_cycle: u32, request_indices: &mut Vec<u32>) {
        self.lookup_hints
            .entry(request_cycle)
            .and_modify(|bus_row| {
                bus_row.send_requests(request_indices);
            })
            .or_insert_with(|| ChipletsBusRow::new(request_indices, None));
    }

    /// Provides lookup data at the specified cycle, which is the row of the Chiplets execution
    /// trace that contains this lookup row.
    fn provide_lookup(&mut self, response_cycle: u32) {
        let response_idx = self.responses.len() as u32;
        self.lookup_hints
            .entry(response_cycle)
            .and_modify(|bus_row| {
                bus_row.send_response(response_idx);
            })
            .or_insert_with(|| ChipletsBusRow::new(&[], Some(response_idx)));
    }

    // HASHER LOOKUPS
    // --------------------------------------------------------------------------------------------

    /// Requests lookups at the specified `cycle` for the initial row and result row of Hash
    /// operations in the Hash Chiplet. This request is expected to originate from operation
    /// executors requesting one or more hash operations for the Stack where all operation lookups
    /// must be included at the same cycle. For simple permutations this will require 2 lookups,
    /// while for a Merkle root update it will require 4, since two Hash operations are required.
    pub(crate) fn request_hasher_operation(&mut self, lookups: &[HasherLookup], cycle: u32) {
        debug_assert!(
            lookups.len() == 2 || lookups.len() == 4,
            "incorrect number of lookup rows for hasher operation request"
        );
        let mut request_indices = vec![0; lookups.len()];
        for (idx, lookup) in lookups.iter().enumerate() {
            request_indices[idx] = self.requests.len() as u32;
            self.requests.push(ChipletLookup::Hasher(*lookup));
        }
        self.request_lookups(cycle, &mut request_indices);
    }

    /// Requests the specified lookup from the Hash Chiplet at the specified `cycle`. Single lookup
    /// requests are expected to originate from the decoder during control block decoding. This
    /// lookup can be for either the initial or the final row of the hash operation.
    pub(crate) fn request_hasher_lookup(&mut self, lookup: HasherLookup, cycle: u32) {
        self.request_lookups(cycle, &mut vec![self.requests.len() as u32]);
        self.requests.push(ChipletLookup::Hasher(lookup));
    }

    /// Adds the request for the specified lookup to a queue from which it can be sent later when
    /// the cycle of the request is known. Queued requests are expected to originate from the
    /// decoder, since the hash is computed at the start of each control block (along with all
    /// required lookups), but the decoder does not request intermediate and final lookups until the
    /// end of the control block or until a `RESPAN`, in the case of `SPAN` blocks with more than
    /// one operation batch.
    pub(crate) fn enqueue_hasher_request(&mut self, lookup: HasherLookup) {
        self.queued_requests.push(lookup);
    }

    /// Pops the top HasherLookup request off the queue and sends it to the bus. This request is
    /// expected to originate from the decoder as it continues or finalizes control blocks with
    /// `RESPAN` or `END`.
    pub(crate) fn send_queued_hasher_request(&mut self, cycle: u32) {
        let lookup = self.queued_requests.pop();
        debug_assert!(lookup.is_some(), "no queued requests");

        if let Some(lookup) = lookup {
            self.request_hasher_lookup(lookup, cycle);
        }
    }

    /// Provides the data of a hash chiplet operation contained in the [Hasher] table. The hash
    /// lookup value is provided at cycle `response_cycle`, which is the row of the execution trace
    /// that contains this Hasher row. It will always be either the first or last row of a Hasher
    /// operation cycle.
    pub(crate) fn provide_hasher_lookup(&mut self, lookup: HasherLookup, response_cycle: u32) {
        self.provide_lookup(response_cycle);
        self.responses.push(ChipletLookup::Hasher(lookup));
    }

    /// Provides multiple hash lookup values and their response cycles, which are the rows of the
    /// execution trace which contains the corresponding hasher row for either the start or end of
    /// a hasher operation cycle.
    pub(crate) fn provide_hasher_lookups(&mut self, lookups: &[HasherLookup]) {
        for lookup in lookups.iter() {
            self.provide_hasher_lookup(*lookup, lookup.cycle());
        }
    }

    // BITWISE LOOKUPS
    // --------------------------------------------------------------------------------------------

    /// Requests the specified bitwise lookup at the specified `cycle`. This request is expected to
    /// originate from operation executors.
    pub(crate) fn request_bitwise_operation(&mut self, lookup: BitwiseLookup, cycle: u32) {
        self.request_lookups(cycle, &mut vec![self.requests.len() as u32]);
        self.requests.push(ChipletLookup::Bitwise(lookup));
    }

    /// Provides the data of a bitwise operation contained in the [Bitwise] table. The bitwise value
    /// is provided at cycle `response_cycle`, which is the row of the execution trace that contains
    /// this Bitwise row. It will always be the final row of a Bitwise operation cycle.
    pub(crate) fn provide_bitwise_operation(&mut self, lookup: BitwiseLookup, response_cycle: u32) {
        self.provide_lookup(response_cycle);
        self.responses.push(ChipletLookup::Bitwise(lookup));
    }

    // MEMORY LOOKUPS
    // --------------------------------------------------------------------------------------------

    /// Sends the specified memory access requests. There must be exactly one or two requests. The
    /// requests are made at the specified `cycle` and are expected to originate from operation
    /// executors.
    pub(crate) fn request_memory_operation(&mut self, lookups: &[MemoryLookup], cycle: u32) {
        debug_assert!(
            lookups.len() == 1 || lookups.len() == 2,
            "invalid number of requested memory operations"
        );
        let mut request_indices = vec![0; lookups.len()];
        for (idx, lookup) in lookups.iter().enumerate() {
            request_indices[idx] = self.requests.len() as u32;
            self.requests.push(ChipletLookup::Memory(*lookup));
        }
        self.request_lookups(cycle, &mut request_indices);
    }

    /// Provides the data of the specified memory access. The memory access data is provided at
    /// cycle `response_cycle`, which is the row of the execution trace that contains this Memory
    /// row.
    pub(crate) fn provide_memory_operation(&mut self, lookup: MemoryLookup, response_cycle: u32) {
        self.provide_lookup(response_cycle);
        self.responses.push(ChipletLookup::Memory(lookup));
    }

    // KERNEL ROM LOOKUPS
    // --------------------------------------------------------------------------------------------

    /// Requests the specified kernel procedure lookup at the specified `cycle`. This request is
    /// expected to originate from operation executors.
    pub(crate) fn request_kernel_proc_call(&mut self, lookup: KernelProcLookup, cycle: u32) {
        self.request_lookups(cycle, &mut vec![self.requests.len() as u32]);
        self.requests.push(ChipletLookup::KernelRom(lookup));
    }

    /// Provides a kernel procedure call contained in the [KernelRom] chiplet. The procedure access
    /// is provided at cycle `response_cycle`, which is the row of the execution trace that contains
    /// this [KernelRom] row.
    pub(crate) fn provide_kernel_proc_call(
        &mut self,
        lookup: KernelProcLookup,
        response_cycle: u32,
    ) {
        self.provide_lookup(response_cycle);
        self.responses.push(ChipletLookup::KernelRom(lookup));
    }

    // AUX TRACE BUILDER GENERATION
    // --------------------------------------------------------------------------------------------

    /// Converts this [ChipletsBus] into an auxiliary trace builder which can be used to construct
    /// the auxiliary trace column describing the [Chiplets] lookups at every cycle.
    pub(crate) fn into_aux_builder(self) -> BusTraceBuilder {
        let lookup_hints = self.lookup_hints.into_iter().collect();

        BusTraceBuilder::new(lookup_hints, self.requests, self.responses)
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns an option with the lookup hint for the specified cycle.
    #[cfg(test)]
    pub(crate) fn get_lookup_hint(&self, cycle: u32) -> Option<&ChipletsBusRow> {
        self.lookup_hints.get(&cycle)
    }

    /// Returns the ith lookup response provided by the Chiplets module.
    #[cfg(test)]
    pub(crate) fn get_response_row(&self, i: usize) -> ChipletLookup {
        self.responses[i].clone()
    }
}

// CHIPLETS LOOKUPS
// ================================================================================================

/// This represents all communication with the Chiplets Bus at a single cycle. Multiple requests can
/// be sent to the bus in any given cycle, but only one response can be provided.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChipletsBusRow {
    requests: Vec<u32>,
    response: Option<u32>,
}

impl ChipletsBusRow {
    pub(crate) fn new(requests: &[u32], response: Option<u32>) -> Self {
        ChipletsBusRow {
            requests: requests.to_vec(),
            response,
        }
    }

    pub(super) fn requests(&self) -> &[u32] {
        &self.requests
    }

    pub(super) fn response(&self) -> Option<u32> {
        self.response
    }

    fn send_requests(&mut self, requests: &mut Vec<u32>) {
        self.requests.append(requests);
    }

    fn send_response(&mut self, response: u32) {
        debug_assert!(self.response.is_none(), "bus row already contains a response");
        self.response = Some(response);
    }
}

/// Data representing a single lookup row in one of the [Chiplets].
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ChipletLookup {
    Bitwise(BitwiseLookup),
    Hasher(HasherLookup),
    KernelRom(KernelProcLookup),
    Memory(MemoryLookup),
}

impl LookupTableRow for ChipletLookup {
    fn to_value<E: FieldElement<BaseField = Felt>>(
        &self,
        main_trace: &ColMatrix<Felt>,
        alphas: &[E],
    ) -> E {
        match self {
            ChipletLookup::Bitwise(row) => row.to_value(main_trace, alphas),
            ChipletLookup::Hasher(row) => row.to_value(main_trace, alphas),
            ChipletLookup::KernelRom(row) => row.to_value(main_trace, alphas),
            ChipletLookup::Memory(row) => row.to_value(main_trace, alphas),
        }
    }
}
