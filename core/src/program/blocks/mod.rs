use super::{Digest, Felt};

mod span_block;
pub use span_block::{
    batch_ops, get_span_op_group_count, OpBatch, BATCH_SIZE as OP_BATCH_SIZE,
    GROUP_SIZE as OP_GROUP_SIZE,
};
