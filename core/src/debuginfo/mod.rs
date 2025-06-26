mod location;
mod source_file;
mod source_manager;
mod span;

use alloc::{collections::btree_map::BTreeMap, sync::Arc, vec::Vec};

#[cfg(feature = "std")]
pub use self::source_manager::SourceManagerExt;
pub use self::{
    location::{FileLineCol, Location},
    source_file::{
        ByteIndex, ByteOffset, ColumnIndex, LineIndex, SourceContent, SourceFile, SourceFileRef,
    },
    source_manager::{DefaultSourceManager, SourceId, SourceManager},
    span::{SourceSpan, Span, Spanned},
};
use crate::{
    mast::DecoratorId,
    operations::{Decorator, OperationId},
};

// #[derive(Debug, Clone, Default, PartialEq, Eq)]
// pub struct DebugInfo {
//     /// List of decorators in the MAST forest such that decorators for the same operation are
// next     /// to each other
//     pub decorators: Vec<Decorator>,
//     /// A map from an operation to a decorator in the `decorators` field.
//     pub op_decorators: BTreeMap<OperationId, (usize, usize)>,
//     /// A map from error codes to error messages.
//     pub error_codes: BTreeMap<u64, Arc<str>>,
// }
// // TODO remove pub and add proper constructor

// impl DebugInfo {
//     pub fn new(error_codes: BTreeMap<u64, Arc<str>>) -> Self {
//         Self {
//             decorators: vec![],
//             op_decorators: BTreeMap::new(),
//             error_codes,
//         }
//     }

//     pub fn get_decorators(&self, op_id: &OperationId) -> &[Decorator] {
//         let (start, end) = self.op_decorators[op_id];
//         &self.decorators[start..end]
//     }

//     pub fn add_decorator(&mut self, op_id: OperationId, decorator: Decorator) {
//         let pos = self.decorators.len();
//         self.decorators.push(decorator);
//         self.op_decorators.insert(op_id, (pos, pos + 1));
//         // TODO increase range
//     }
// }

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DebugInfo {
    pub decorators: Vec<Decorator>,
    pub op_decorators: BTreeMap<OperationId, (Vec<usize>, Vec<usize>)>,
    pub error_codes: BTreeMap<u64, Arc<str>>,
}
// TODO remove pub and add proper constructor

impl DebugInfo {
    pub fn new(error_codes: BTreeMap<u64, Arc<str>>) -> Self {
        Self {
            decorators: Vec::new(),
            op_decorators: BTreeMap::new(),
            error_codes,
        }
    }

    pub fn get_decorator_ids_before(&self, op_id: &OperationId) -> Option<&Vec<usize>> {
        self.op_decorators.get(op_id).map(|(before, _)| before)
    }

    pub fn get_decorator_ids_after(&self, op_id: &OperationId) -> Option<&Vec<usize>> {
        self.op_decorators.get(op_id).map(|(_, after)| after)
    }

    pub fn add_decorator_id(
        &mut self,
        op_id: OperationId,
        decorator_id: DecoratorId,
        before: bool,
    ) {
        self.op_decorators
            .entry(op_id)
            .and_modify(|(before_decs, after_decs)| {
                let decs = if before { before_decs } else { after_decs };
                decs.push(decorator_id.into())
            })
            .or_insert(if before {
                (vec![decorator_id.into()], vec![])
            } else {
                (vec![], vec![decorator_id.into()])
            });
    }

    // pub fn add_decorator(&mut self, op_id: OperationId, decorator: Decorator, after: bool) {
    //     let pos = self.decorators.len();
    //     self.decorators.push(decorator);
    //     self.op_decorators
    //         .entry(op_id)
    //         .and_modify(|decs| decs.push(pos))
    //         .or_insert(vec![pos]);
    // }
}
