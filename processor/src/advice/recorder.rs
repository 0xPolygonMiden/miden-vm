use super::{
    AdviceInputs, Felt, KvMap, RecordingMap, RecordingMerkleMap, RecordingMerkleStore,
    StackMapStoreProvider,
};

// RECORDING ADVICE PROVIDER
// ================================================================================================

/// An in-memory `[AdviceProvider]` implementation to support program execution with support for
/// data access recording. The recorder can be converted into a proof which can be used to provide
/// the non-deterministic inputs for program execution.
///
/// Uses [RecordingMap] and [RecordingMerkleStore] as data backends.
#[cfg(not(any(test, feature = "internals")))]
#[derive(Debug, Clone, Default)]
pub struct RecAdviceProvider {
    step: u32,
    stack: Vec<Felt>,
    init_stack: Vec<Felt>,
    map: RecordingMap<[u8; 32], Vec<Felt>>,
    store: RecordingMerkleStore,
}

impl From<AdviceInputs> for RecAdviceProvider {
    fn from(inputs: AdviceInputs) -> Self {
        let (mut stack, map, store) = inputs.into_parts();
        let init_stack = stack.clone();
        stack.reverse();
        Self {
            step: 0,
            stack,
            init_stack,
            map: RecordingMap::new(map),
            store: store.into(),
        }
    }
}

impl StackMapStoreProvider for &mut RecAdviceProvider {
    type Map = RecordingMerkleMap;

    fn get_step(&self) -> u32 {
        self.step
    }

    fn get_step_mut(&mut self) -> &mut u32 {
        &mut self.step
    }

    fn get_stack(&self) -> &[Felt] {
        &self.stack
    }

    fn get_stack_mut(&mut self) -> &mut Vec<Felt> {
        &mut self.stack
    }

    fn get_map(&self) -> &dyn KvMap<[u8; 32], Vec<Felt>> {
        &self.map
    }

    fn get_map_mut(&mut self) -> &mut dyn KvMap<[u8; 32], Vec<Felt>> {
        &mut self.map
    }

    fn get_store(&self) -> &RecordingMerkleStore {
        &self.store
    }

    fn get_store_mut(&mut self) -> &mut RecordingMerkleStore {
        &mut self.store
    }
}

impl RecAdviceProvider {
    /// Consumes the [AdviceRecorder] and returns a [AdviceInputs] instance which can be used to
    /// re-execute the program. The returned [AdviceInputs] instance will only contain the
    /// non-deterministic inputs which were requested during program execution.
    pub fn into_proof(self) -> AdviceInputs {
        let Self {
            step: _,
            stack: _,
            init_stack,
            map,
            store,
        } = self;

        let map = map.into_proof();
        let store = store.into_proof();

        AdviceInputs::default()
            .with_stack(init_stack)
            .with_map(map)
            .with_merkle_store(store.into())
    }
}

// INTERNALS
// ================================================================================================

#[cfg(any(test, feature = "internals"))]
#[derive(Debug, Clone, Default)]
pub struct RecAdviceProvider {
    pub step: u32,
    pub stack: Vec<Felt>,
    pub init_stack: Vec<Felt>,
    pub map: RecordingMap<[u8; 32], Vec<Felt>>,
    pub store: RecordingMerkleStore,
}
