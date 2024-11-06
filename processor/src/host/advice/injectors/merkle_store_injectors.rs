use super::super::{AdviceProvider, ExecutionError, HostResponse, ProcessState};

pub(crate) fn update_operand_stack_merkle_node<A: AdviceProvider>(
    advice_provider: &mut A,
    process: ProcessState,
) -> Result<HostResponse, ExecutionError> {
    let depth = process.get_stack_item(4);
    let index = process.get_stack_item(5);
    let old_root = [
        process.get_stack_item(9),
        process.get_stack_item(8),
        process.get_stack_item(7),
        process.get_stack_item(6),
    ];
    let new_node = [
        process.get_stack_item(13),
        process.get_stack_item(12),
        process.get_stack_item(11),
        process.get_stack_item(10),
    ];
    let (path, _) = advice_provider.update_merkle_node(old_root, &depth, &index, new_node)?;
    Ok(HostResponse::MerklePath(path))
}
