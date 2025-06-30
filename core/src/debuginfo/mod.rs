mod decorator;
mod location;
mod source_file;
mod source_manager;
mod span;
mod string_table;

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

use alloc::string::{String, ToString};

use decorator::{DecoratorDataBuilder, DecoratorInfo};
use string_table::StringTable;
use winter_utils::{ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable};

impl Serializable for DebugInfo {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        let decorator_count = self.decorators.len();
        target.write_usize(decorator_count);

        let mut decorator_data_builder = DecoratorDataBuilder::new();
        for decorator in &self.decorators {
            decorator_data_builder.add_decorator(decorator)
        }

        let (decorator_data, decorator_infos, string_table) = decorator_data_builder.finalize();

        // decorator data buffers
        decorator_data.write_into(target);
        string_table.write_into(target);

        // Write decorator infos
        for decorator_info in decorator_infos {
            decorator_info.write_into(target);
        }

        self.op_decorators.write_into(target);
        let error_codes: BTreeMap<u64, String> =
            self.error_codes.iter().map(|(k, v)| (*k, v.to_string())).collect();
        error_codes.write_into(target);
    }
}

impl Deserializable for DebugInfo {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let decorator_count = source.read_usize()?;
        let decorator_data: Vec<u8> = Deserializable::read_from(source)?;
        let string_table: StringTable = Deserializable::read_from(source)?;
        let decorator_infos = decorator_infos_iter(source, decorator_count);

        let mut decorators: Vec<Decorator> = vec![];
        for decorator_info in decorator_infos {
            let decorator_info = decorator_info?;
            decorators.push(decorator_info.try_into_decorator(&string_table, &decorator_data)?);
        }

        let op_decorators: BTreeMap<OperationId, (Vec<usize>, Vec<usize>)> =
            Deserializable::read_from(source)?;

        let error_codes: BTreeMap<u64, String> = Deserializable::read_from(source)?;
        let error_codes: BTreeMap<u64, Arc<str>> =
            error_codes.into_iter().map(|(k, v)| (k, Arc::from(v))).collect();

        Ok(DebugInfo { decorators, op_decorators, error_codes })
    }
}

fn decorator_infos_iter<'a, R>(
    source: &'a mut R,
    decorator_count: usize,
) -> impl Iterator<Item = Result<DecoratorInfo, DeserializationError>> + 'a
where
    R: ByteReader + 'a,
{
    let mut remaining = decorator_count;
    core::iter::from_fn(move || {
        if remaining == 0 {
            return None;
        }
        remaining -= 1;
        Some(DecoratorInfo::read_from(source))
    })
}
