use alloc::collections::BTreeMap;
use core::borrow::Borrow;

use super::MetaExpr;
use crate::{SourceSpan, Spanned, ast::Ident};

/// Represents the metadata of a key-value [crate::ast::Attribute], i.e. `@props(key = value)`
#[derive(Clone)]
pub struct MetaKeyValue {
    pub span: SourceSpan,
    /// The name of the key-value dictionary
    pub name: Ident,
    /// The set of key-value pairs provided as arguments to this attribute
    pub items: BTreeMap<Ident, MetaExpr>,
}

impl Spanned for MetaKeyValue {
    #[inline(always)]
    fn span(&self) -> SourceSpan {
        self.span
    }
}

impl MetaKeyValue {
    pub fn new<K, V, I>(name: Ident, items: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<Ident>,
        V: Into<MetaExpr>,
    {
        let items = items.into_iter().map(|(k, v)| (k.into(), v.into())).collect();
        Self { span: SourceSpan::default(), name, items }
    }

    pub fn with_span(mut self, span: SourceSpan) -> Self {
        self.span = span;
        self
    }

    /// Get the name of this metadata as a string
    #[inline]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Get the name of this metadata as an [Ident]
    #[inline]
    pub fn id(&self) -> Ident {
        self.name.clone()
    }

    /// Returns true if this metadata contains an entry for `key`
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        Ident: Borrow<Q> + Ord,
        Q: ?Sized + Ord,
    {
        self.items.contains_key(key)
    }

    /// Returns the value associated with `key`, if present in this metadata
    pub fn get<Q>(&self, key: &Q) -> Option<&MetaExpr>
    where
        Ident: Borrow<Q> + Ord,
        Q: ?Sized + Ord,
    {
        self.items.get(key)
    }

    /// Inserts a new key-value entry in this metadata
    pub fn insert(&mut self, key: impl Into<Ident>, value: impl Into<MetaExpr>) {
        self.items.insert(key.into(), value.into());
    }

    /// Removes the entry associated with `key`, if present in this metadata, and returns it
    pub fn remove<Q>(&mut self, key: &Q) -> Option<MetaExpr>
    where
        Ident: Borrow<Q> + Ord,
        Q: ?Sized + Ord,
    {
        self.items.remove(key)
    }

    /// Get an entry in the key-value map of this metadata for `key`
    pub fn entry(
        &mut self,
        key: Ident,
    ) -> alloc::collections::btree_map::Entry<'_, Ident, MetaExpr> {
        self.items.entry(key)
    }

    /// Get an iterator over the the key-value items of this metadata
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (&Ident, &MetaExpr)> {
        self.items.iter()
    }
}

impl IntoIterator for MetaKeyValue {
    type Item = (Ident, MetaExpr);
    type IntoIter = alloc::collections::btree_map::IntoIter<Ident, MetaExpr>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl Eq for MetaKeyValue {}

impl PartialEq for MetaKeyValue {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.items == other.items
    }
}

impl PartialOrd for MetaKeyValue {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MetaKeyValue {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.name.cmp(&other.name).then_with(|| self.items.cmp(&other.items))
    }
}

impl core::hash::Hash for MetaKeyValue {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.items.hash(state);
    }
}
