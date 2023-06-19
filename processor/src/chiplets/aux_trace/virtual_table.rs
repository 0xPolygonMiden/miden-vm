use super::{ColMatrix, Felt, FieldElement, StarkField, Word};
use crate::trace::LookupTableRow;

// CHIPLETS VIRTUAL TABLE
// ================================================================================================

/// Describes updates to the chiplets virtual table. This includes management of the "sibling table"
/// used by the hasher chiplet and the "kernel procedure table" used by the kernel ROM chiplet.
///
/// - The sibling table is used to enforce Merkle root update computations. The internal u32 values
/// are indices of added/removed rows in a list of rows sorted chronologically (i.e., from first
/// added row to last).
/// - The kernel procedure table contains all kernel procedures along with the address where they
/// first appear in the kernel ROM trace. Each kernel procedure is expected to be included exactly
/// once, regardless of whether it is ever called or not.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ChipletsVTableUpdate {
    SiblingAdded(u32),
    SiblingRemoved(u32),
    KernelProcAdded(u32),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ChipletsVTableRow {
    sibling: Option<SiblingTableRow>,
    kernel_proc: Option<KernelProc>,
}

impl ChipletsVTableRow {
    pub fn new_sibling(index: Felt, sibling: Word) -> Self {
        Self {
            sibling: Some(SiblingTableRow::new(index, sibling)),
            kernel_proc: None,
        }
    }

    pub fn new_kernel_proc(addr: Felt, proc_hash: Word) -> Self {
        Self {
            sibling: None,
            kernel_proc: Some(KernelProc::new(addr, proc_hash)),
        }
    }

    #[cfg(test)]
    pub fn kernel_proc(&self) -> Option<KernelProc> {
        self.kernel_proc
    }
}

impl LookupTableRow for ChipletsVTableRow {
    /// Reduces this row to a single field element in the field specified by E. This requires
    /// at least 6 alpha values.
    fn to_value<E: FieldElement<BaseField = Felt>>(
        &self,
        main_trace: &ColMatrix<Felt>,
        alphas: &[E],
    ) -> E {
        if let Some(sibling) = self.sibling {
            debug_assert!(
                self.kernel_proc.is_none(),
                "a chiplet virtual table row cannot represent both a sibling and a kernel ROM procedure"
            );
            sibling.to_value(main_trace, alphas)
        } else if let Some(kernel_proc) = self.kernel_proc {
            kernel_proc.to_value(main_trace, alphas)
        } else {
            E::ONE
        }
    }
}

// SIBLING TABLE ROW
// ================================================================================================

/// Describes a single entry in the sibling table which consists of a tuple `(index, node)` where
/// index is the index of the node at its depth. For example, assume a leaf has index n. For the
/// leaf's parent the index will be n << 1. For the parent of the parent, the index will be
/// n << 2 etc.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SiblingTableRow {
    index: Felt,
    sibling: Word,
}

impl SiblingTableRow {
    pub fn new(index: Felt, sibling: Word) -> Self {
        Self { index, sibling }
    }
}

impl LookupTableRow for SiblingTableRow {
    /// Reduces this row to a single field element in the field specified by E. This requires
    /// at least 6 alpha values.
    fn to_value<E: FieldElement<BaseField = Felt>>(
        &self,
        _main_trace: &ColMatrix<Felt>,
        alphas: &[E],
    ) -> E {
        // when the least significant bit of the index is 0, the sibling will be in the 3rd word
        // of the hasher state, and when the least significant bit is 1, it will be in the 2nd
        // word. we compute the value in this way to make constraint evaluation a bit easier since
        // we need to compute the 2nd and the 3rd word values for other purposes as well.
        let lsb = self.index.as_int() & 1;
        if lsb == 0 {
            alphas[0]
                + alphas[3].mul_base(self.index)
                + alphas[12].mul_base(self.sibling[0])
                + alphas[13].mul_base(self.sibling[1])
                + alphas[14].mul_base(self.sibling[2])
                + alphas[15].mul_base(self.sibling[3])
        } else {
            alphas[0]
                + alphas[3].mul_base(self.index)
                + alphas[8].mul_base(self.sibling[0])
                + alphas[9].mul_base(self.sibling[1])
                + alphas[10].mul_base(self.sibling[2])
                + alphas[11].mul_base(self.sibling[3])
        }
    }
}

// KERNEL ROM PROCEDURES
// ================================================================================================

/// Describes a single entry in the kernel rom procedure table which consists of a tuple
/// `(addr, proc_hash)` where `addr` is the address of the first entry of the procedure in the
/// kernel ROM table and `proc_hash` is the 4-element root hash of the procedure.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct KernelProc {
    addr: Felt,
    proc_hash: Word,
}

impl KernelProc {
    pub fn new(addr: Felt, proc_hash: Word) -> Self {
        Self { addr, proc_hash }
    }

    #[cfg(test)]
    pub fn proc_hash(&self) -> Word {
        self.proc_hash
    }
}

impl LookupTableRow for KernelProc {
    /// Reduces this row to a single field element in the field specified by E. This requires
    /// at least 6 alpha values.
    fn to_value<E: FieldElement<BaseField = Felt>>(
        &self,
        _main_trace: &ColMatrix<Felt>,
        alphas: &[E],
    ) -> E {
        alphas[0]
            + alphas[1].mul_base(self.addr)
            + alphas[2].mul_base(self.proc_hash[0])
            + alphas[3].mul_base(self.proc_hash[1])
            + alphas[4].mul_base(self.proc_hash[2])
            + alphas[5].mul_base(self.proc_hash[3])
    }
}
