use super::{AssemblyError, Operation, Vec};
use vm_core::utils::PushMany;

// STACK MANIPULATION
// ================================================================================================

/// Translates drop assembly instruction to VM operation DROP.
pub fn parse_drop(span_ops: &mut Vec<Operation>) -> Result<(), AssemblyError> {
    span_ops.push(Operation::Drop);
    Ok(())
}

/// Translates dropw assembly instruction to VM operations DROP DROP DROP DROP.
pub fn parse_dropw(span_ops: &mut Vec<Operation>) -> Result<(), AssemblyError> {
    span_ops.push_many(Operation::Drop, 4);
    Ok(())
}

/// Translates padw assembly instruction to VM operations PAD PAD PAD PAD.
pub fn parse_padw(span_ops: &mut Vec<Operation>) -> Result<(), AssemblyError> {
    span_ops.push_many(Operation::Pad, 4);
    Ok(())
}

/// Translates dup.n assembly instruction to VM operations DUPN.
pub fn parse_dup(span_ops: &mut Vec<Operation>, dup_num: u8) -> Result<(), AssemblyError> {
    match dup_num {
        0 => span_ops.push(Operation::Dup0),
        1 => span_ops.push(Operation::Dup1),
        2 => span_ops.push(Operation::Dup2),
        3 => span_ops.push(Operation::Dup3),
        4 => span_ops.push(Operation::Dup4),
        5 => span_ops.push(Operation::Dup5),
        6 => span_ops.push(Operation::Dup6),
        7 => span_ops.push(Operation::Dup7),
        8 => {
            span_ops.push(Operation::Pad);
            span_ops.push(Operation::Dup9);
            span_ops.push(Operation::Add);
        }
        9 => span_ops.push(Operation::Dup9),
        10 => {
            span_ops.push(Operation::Pad);
            span_ops.push(Operation::Dup11);
            span_ops.push(Operation::Add);
        }
        11 => span_ops.push(Operation::Dup11),
        12 => {
            span_ops.push(Operation::Pad);
            span_ops.push(Operation::Dup13);
            span_ops.push(Operation::Add);
        }
        13 => span_ops.push(Operation::Dup13),
        14 => {
            span_ops.push(Operation::Pad);
            span_ops.push(Operation::Dup15);
            span_ops.push(Operation::Add);
        }
        15 => span_ops.push(Operation::Dup15),
        _ => unreachable!(),
    };

    Ok(())
}

/// Translates dupw.n assembly instruction to four VM operations DUP depending on
/// the index of the word.
pub fn parse_dupw(span_ops: &mut Vec<Operation>, dupw_num: usize) -> Result<(), AssemblyError> {
    match dupw_num {
        0 => {
            span_ops.push(Operation::Dup3);
            span_ops.push(Operation::Dup3);
            span_ops.push(Operation::Dup3);
            span_ops.push(Operation::Dup3);
        }
        1 => {
            span_ops.push(Operation::Dup7);
            span_ops.push(Operation::Dup7);
            span_ops.push(Operation::Dup7);
            span_ops.push(Operation::Dup7);
        }
        2 => {
            span_ops.push(Operation::Dup11);
            span_ops.push(Operation::Dup11);
            span_ops.push(Operation::Dup11);
            span_ops.push(Operation::Dup11);
        }
        3 => {
            span_ops.push(Operation::Dup15);
            span_ops.push(Operation::Dup15);
            span_ops.push(Operation::Dup15);
            span_ops.push(Operation::Dup15);
        }
        _ => unreachable!(),
    }

    Ok(())
}

/// Translates swap.x assembly instruction to VM operations MOVUPX MOVDN(X-1)
pub fn parse_swap(span_ops: &mut Vec<Operation>, swap_num: u8) -> Result<(), AssemblyError> {
    match swap_num {
        1 => span_ops.push(Operation::Swap),
        2 => {
            span_ops.push(Operation::Swap);
            span_ops.push(Operation::MovUp2);
        }
        3 => {
            span_ops.push(Operation::MovDn2);
            span_ops.push(Operation::MovUp3);
        }
        4 => {
            span_ops.push(Operation::MovDn3);
            span_ops.push(Operation::MovUp4);
        }
        5 => {
            span_ops.push(Operation::MovDn4);
            span_ops.push(Operation::MovUp5);
        }
        6 => {
            span_ops.push(Operation::MovDn5);
            span_ops.push(Operation::MovUp6);
        }
        7 => {
            span_ops.push(Operation::MovDn6);
            span_ops.push(Operation::MovUp7);
        }
        8 => {
            span_ops.push(Operation::MovDn7);
            span_ops.push(Operation::MovUp8);
        }
        9 => {
            span_ops.push(Operation::MovDn8);
            span_ops.push(Operation::SwapDW);
            span_ops.push(Operation::Swap);
            span_ops.push(Operation::SwapDW);
            span_ops.push(Operation::MovUp8);
        }
        10 => {
            span_ops.push(Operation::MovDn8);
            span_ops.push(Operation::SwapDW);
            span_ops.push(Operation::Swap);
            span_ops.push(Operation::MovUp2);
            span_ops.push(Operation::SwapDW);
            span_ops.push(Operation::MovUp8);
        }
        11 => {
            span_ops.push(Operation::MovDn8);
            span_ops.push(Operation::SwapDW);
            span_ops.push(Operation::MovDn2);
            span_ops.push(Operation::MovUp3);
            span_ops.push(Operation::SwapDW);
            span_ops.push(Operation::MovUp8);
        }
        12 => {
            span_ops.push(Operation::MovDn8);
            span_ops.push(Operation::SwapDW);
            span_ops.push(Operation::MovDn3);
            span_ops.push(Operation::MovUp4);
            span_ops.push(Operation::SwapDW);
            span_ops.push(Operation::MovUp8);
        }
        13 => {
            span_ops.push(Operation::MovDn8);
            span_ops.push(Operation::SwapDW);
            span_ops.push(Operation::MovDn4);
            span_ops.push(Operation::MovUp5);
            span_ops.push(Operation::SwapDW);
            span_ops.push(Operation::MovUp8);
        }
        14 => {
            span_ops.push(Operation::MovDn8);
            span_ops.push(Operation::SwapDW);
            span_ops.push(Operation::MovDn5);
            span_ops.push(Operation::MovUp6);
            span_ops.push(Operation::SwapDW);
            span_ops.push(Operation::MovUp8);
        }
        15 => {
            span_ops.push(Operation::MovDn8);
            span_ops.push(Operation::SwapDW);
            span_ops.push(Operation::MovDn6);
            span_ops.push(Operation::MovUp7);
            span_ops.push(Operation::SwapDW);
            span_ops.push(Operation::MovUp8);
        }
        _ => unreachable!(),
    }

    Ok(())
}

/// Translates swapw.n assembly instruction to four VM operation SWAPWN
pub fn parse_swapw(span_ops: &mut Vec<Operation>, swapw_num: u8) -> Result<(), AssemblyError> {
    match swapw_num {
        1 => span_ops.push(Operation::SwapW),
        2 => span_ops.push(Operation::SwapW2),
        3 => span_ops.push(Operation::SwapW3),
        _ => unreachable!(),
    };

    Ok(())
}

/// Translates swapdw assembly instruction to four VM SWAPW operations
pub fn parse_swapdw(span_ops: &mut Vec<Operation>) -> Result<(), AssemblyError> {
    span_ops.push(Operation::SwapDW);
    Ok(())
}

/// Translates movup.x assembly instruction to VM operations.
/// We specifically utilize the MovUpX VM operations for indexes that match
/// exactly with the assembly instruction.
/// The reamaining ones we implement them PAD MOVUPX ADD.
pub fn parse_movup(span_ops: &mut Vec<Operation>, movup_num: u8) -> Result<(), AssemblyError> {
    match movup_num {
        2 => span_ops.push(Operation::MovUp2),
        3 => span_ops.push(Operation::MovUp3),
        4 => span_ops.push(Operation::MovUp4),
        5 => span_ops.push(Operation::MovUp5),
        6 => span_ops.push(Operation::MovUp6),
        7 => span_ops.push(Operation::MovUp7),
        8 => span_ops.push(Operation::MovUp8),
        9 => {
            span_ops.push(Operation::SwapDW);
            span_ops.push(Operation::Swap);
            span_ops.push(Operation::SwapDW);
            span_ops.push(Operation::MovUp8);
        }
        10 => {
            span_ops.push(Operation::SwapDW);
            span_ops.push(Operation::MovUp2);
            span_ops.push(Operation::SwapDW);
            span_ops.push(Operation::MovUp8);
        }
        11 => {
            span_ops.push(Operation::SwapDW);
            span_ops.push(Operation::MovUp3);
            span_ops.push(Operation::SwapDW);
            span_ops.push(Operation::MovUp8);
        }
        12 => {
            span_ops.push(Operation::SwapDW);
            span_ops.push(Operation::MovUp4);
            span_ops.push(Operation::SwapDW);
            span_ops.push(Operation::MovUp8);
        }
        13 => {
            span_ops.push(Operation::SwapDW);
            span_ops.push(Operation::MovUp5);
            span_ops.push(Operation::SwapDW);
            span_ops.push(Operation::MovUp8);
        }
        14 => {
            span_ops.push(Operation::SwapDW);
            span_ops.push(Operation::MovUp6);
            span_ops.push(Operation::SwapDW);
            span_ops.push(Operation::MovUp8);
        }
        15 => {
            span_ops.push(Operation::SwapDW);
            span_ops.push(Operation::MovUp7);
            span_ops.push(Operation::SwapDW);
            span_ops.push(Operation::MovUp8);
        }
        _ => unreachable!(),
    };

    Ok(())
}

/// Translates movupw.x assembly instruction to VM operations.
///
/// Specifically:
/// * movupw.2 is translated into SWAPW SWAPW2
/// * movupw.3 is translated into SWAPW SWAPW2 SWAPW3
pub fn parse_movupw(span_ops: &mut Vec<Operation>, movupw_num: u8) -> Result<(), AssemblyError> {
    match movupw_num {
        2 => {
            span_ops.push(Operation::SwapW);
            span_ops.push(Operation::SwapW2);
        }
        3 => {
            span_ops.push(Operation::SwapW);
            span_ops.push(Operation::SwapW2);
            span_ops.push(Operation::SwapW3);
        }
        _ => unreachable!(),
    };

    Ok(())
}

/// Translates movdn.x assembly instruction to VM operations.
/// We specifically utilize the MovDnX VM operations for indexes that match
/// exactly with the assembly instruction.
/// The reamaining ones we implement them PAD SWAP MOVDNX DROP.
pub fn parse_movdn(span_ops: &mut Vec<Operation>, movdn_num: u8) -> Result<(), AssemblyError> {
    match movdn_num {
        2 => span_ops.push(Operation::MovDn2),
        3 => span_ops.push(Operation::MovDn3),
        4 => span_ops.push(Operation::MovDn4),
        5 => span_ops.push(Operation::MovDn5),
        6 => span_ops.push(Operation::MovDn6),
        7 => span_ops.push(Operation::MovDn7),
        8 => span_ops.push(Operation::MovDn8),
        9 => {
            span_ops.push(Operation::MovDn8);
            span_ops.push(Operation::SwapDW);
            span_ops.push(Operation::Swap);
            span_ops.push(Operation::SwapDW);
        }
        10 => {
            span_ops.push(Operation::MovDn8);
            span_ops.push(Operation::SwapDW);
            span_ops.push(Operation::MovDn2);
            span_ops.push(Operation::SwapDW);
        }
        11 => {
            span_ops.push(Operation::MovDn8);
            span_ops.push(Operation::SwapDW);
            span_ops.push(Operation::MovDn3);
            span_ops.push(Operation::SwapDW);
        }
        12 => {
            span_ops.push(Operation::MovDn8);
            span_ops.push(Operation::SwapDW);
            span_ops.push(Operation::MovDn4);
            span_ops.push(Operation::SwapDW);
        }
        13 => {
            span_ops.push(Operation::MovDn8);
            span_ops.push(Operation::SwapDW);
            span_ops.push(Operation::MovDn5);
            span_ops.push(Operation::SwapDW);
        }
        14 => {
            span_ops.push(Operation::MovDn8);
            span_ops.push(Operation::SwapDW);
            span_ops.push(Operation::MovDn6);
            span_ops.push(Operation::SwapDW);
        }
        15 => {
            span_ops.push(Operation::MovDn8);
            span_ops.push(Operation::SwapDW);
            span_ops.push(Operation::MovDn7);
            span_ops.push(Operation::SwapDW);
        }
        _ => unreachable!(),
    };

    Ok(())
}

/// Translates movdnw.x assembly instruction to VM operations.
///
/// Specifically:
/// * movdnw.2 is translated into SWAPW2 SWAPW
/// * movdnw.3 is translated into SWAPW3 SWAPW2 SWAPW
pub fn parse_movdnw(span_ops: &mut Vec<Operation>, movdnw_num: u8) -> Result<(), AssemblyError> {
    match movdnw_num {
        2 => {
            span_ops.push(Operation::SwapW2);
            span_ops.push(Operation::SwapW);
        }
        3 => {
            span_ops.push(Operation::SwapW3);
            span_ops.push(Operation::SwapW2);
            span_ops.push(Operation::SwapW);
        }
        _ => unreachable!(),
    };

    Ok(())
}

// CONDITIONAL MANIPULATION
// ================================================================================================

/// Translates cswap assembly instruction that translates to CSWAP.
pub fn parse_cswap(span_ops: &mut Vec<Operation>) -> Result<(), AssemblyError> {
    span_ops.push(Operation::CSwap);
    Ok(())
}

/// Translates cswapw assembly instruction that translates to CSWAPW.
pub fn parse_cswapw(span_ops: &mut Vec<Operation>) -> Result<(), AssemblyError> {
    span_ops.push(Operation::CSwapW);
    Ok(())
}

/// Translates cdrop assembly instruction that translates to CSWAP DROP
pub fn parse_cdrop(span_ops: &mut Vec<Operation>) -> Result<(), AssemblyError> {
    span_ops.push(Operation::CSwap);
    span_ops.push(Operation::Drop);
    Ok(())
}

/// Translates cdropw assembly instruction that translates to CSWAPW DROP DROP DROP DROP
pub fn parse_cdropw(span_ops: &mut Vec<Operation>) -> Result<(), AssemblyError> {
    span_ops.push(Operation::CSwapW);
    span_ops.push(Operation::Drop);
    span_ops.push(Operation::Drop);
    span_ops.push(Operation::Drop);
    span_ops.push(Operation::Drop);
    Ok(())
}
