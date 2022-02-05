use super::{Felt, FieldElement, Selectors};

pub const LINEAR_HASH: Selectors = [Felt::ONE, Felt::ZERO, Felt::ZERO];
pub const MP_VERIFY: Selectors = [Felt::ONE, Felt::ZERO, Felt::ONE];
pub const MR_UPDATE_OLD: Selectors = [Felt::ONE, Felt::ONE, Felt::ZERO];
pub const MR_UPDATE_NEW: Selectors = [Felt::ONE, Felt::ONE, Felt::ONE];

//pub const CONTINUE_LINEAR_HASH: Selectors = [Felt::ZERO, Felt::ZERO, Felt::ZERO],
pub const CONTINUE_MP_VERIFY: Selectors = [Felt::ZERO, Felt::ZERO, Felt::ONE];
pub const CONTINUE_MR_UPDATE_OLD: Selectors = [Felt::ZERO, Felt::ONE, Felt::ZERO];
pub const CONTINUE_MR_UPDATE_NEW: Selectors = [Felt::ZERO, Felt::ONE, Felt::ONE];

pub const RETURN_HASH: Selectors = [Felt::ZERO, Felt::ZERO, Felt::ZERO];
pub const RETURN_STATE: Selectors = [Felt::ZERO, Felt::ZERO, Felt::ONE];
