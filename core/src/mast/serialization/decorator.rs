use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::ToPrimitive;

use crate::{AdviceInjector, DebugOptions, Decorator};

/// TODOP: Document
#[derive(FromPrimitive, ToPrimitive)]
#[repr(u8)]
pub enum EncodedDecoratorVariant {
    AdviceInjectorMerkleNodeMerge,
    AdviceInjectorMerkleNodeToStack,
    AdviceInjectorUpdateMerkleNode,
    AdviceInjectorMapValueToStack,
    AdviceInjectorU64Div,
    AdviceInjectorExt2Inv,
    AdviceInjectorExt2Intt,
    AdviceInjectorSmtGet,
    AdviceInjectorSmtSet,
    AdviceInjectorSmtPeek,
    AdviceInjectorU32Clz,
    AdviceInjectorU32Ctz,
    AdviceInjectorU32Clo,
    AdviceInjectorU32Cto,
    AdviceInjectorILog2,
    AdviceInjectorMemToMap,
    AdviceInjectorHdwordToMap,
    AdviceInjectorHpermToMap,
    AdviceInjectorSigToStack,
    AssemblyOp,
    DebugOptionsStackAll,
    DebugOptionsStackTop,
    DebugOptionsMemAll,
    DebugOptionsMemInterval,
    DebugOptionsLocalInterval,
    Event,
    Trace,
}

impl EncodedDecoratorVariant {
    pub fn discriminant(&self) -> u8 {
        self.to_u8().expect("guaranteed to fit in a `u8` due to #[repr(u8)]")
    }
}

impl From<&Decorator> for EncodedDecoratorVariant {
    fn from(decorator: &Decorator) -> Self {
        match decorator {
            Decorator::Advice(advice_injector) => match advice_injector {
                AdviceInjector::MerkleNodeMerge => Self::AdviceInjectorMerkleNodeMerge,
                AdviceInjector::MerkleNodeToStack => Self::AdviceInjectorMerkleNodeToStack,
                AdviceInjector::UpdateMerkleNode => todo!(),
                AdviceInjector::MapValueToStack {
                    include_len: _,
                    key_offset: _,
                } => Self::AdviceInjectorMapValueToStack,
                AdviceInjector::U64Div => Self::AdviceInjectorU64Div,
                AdviceInjector::Ext2Inv => Self::AdviceInjectorExt2Inv,
                AdviceInjector::Ext2Intt => Self::AdviceInjectorExt2Intt,
                AdviceInjector::SmtGet => Self::AdviceInjectorSmtGet,
                AdviceInjector::SmtSet => Self::AdviceInjectorSmtSet,
                AdviceInjector::SmtPeek => Self::AdviceInjectorSmtPeek,
                AdviceInjector::U32Clz => Self::AdviceInjectorU32Clz,
                AdviceInjector::U32Ctz => Self::AdviceInjectorU32Ctz,
                AdviceInjector::U32Clo => Self::AdviceInjectorU32Clo,
                AdviceInjector::U32Cto => Self::AdviceInjectorU32Cto,
                AdviceInjector::ILog2 => Self::AdviceInjectorILog2,
                AdviceInjector::MemToMap => Self::AdviceInjectorMemToMap,
                AdviceInjector::HdwordToMap { domain: _ } => Self::AdviceInjectorHdwordToMap,
                AdviceInjector::HpermToMap => Self::AdviceInjectorHpermToMap,
                AdviceInjector::SigToStack { kind: _ } => Self::AdviceInjectorSigToStack,
            },
            Decorator::AsmOp(_) => Self::AssemblyOp,
            Decorator::Debug(debug_options) => match debug_options {
                DebugOptions::StackAll => Self::DebugOptionsStackAll,
                DebugOptions::StackTop(_) => Self::DebugOptionsStackTop,
                DebugOptions::MemAll => Self::DebugOptionsMemAll,
                DebugOptions::MemInterval(_, _) => Self::DebugOptionsMemInterval,
                DebugOptions::LocalInterval(_, _, _) => Self::DebugOptionsLocalInterval,
            },
            Decorator::Event(_) => Self::Event,
            Decorator::Trace(_) => Self::Trace,
        }
    }
}
