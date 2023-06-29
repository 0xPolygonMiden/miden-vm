use super::{
    ByteReader, ByteWriter, Deserializable, DeserializationError, Node, Serializable,
    SourceLocation, Vec,
};
use core::{iter, slice};

// CODE BODY
// ================================================================================================

/// A contiguous sequence of [Node]s with optional [SourceLocation] specified for each node.
///
/// When present, the number of locations is equal to the number of nodes + 1. This is because the
/// last location tracks the `end` token of a body which does not have its own node.
#[derive(Clone, Default, Eq, Debug)]
pub struct CodeBody {
    nodes: Vec<Node>,
    locations: Vec<SourceLocation>,
}

impl CodeBody {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------

    /// Creates a new instance of [CodeBody] populated with the provided `nodes`.
    pub fn new<N>(nodes: N) -> Self
    where
        N: IntoIterator<Item = Node>,
    {
        Self {
            nodes: nodes.into_iter().collect(),
            locations: Vec::new(),
        }
    }

    /// Binds [SourceLocation]s to their respective [Node].
    ///
    /// It is expected that `locations` have the length one greater than the length of `self.nodes`.
    pub fn with_source_locations<L>(mut self, locations: L) -> Self
    where
        L: IntoIterator<Item = SourceLocation>,
    {
        self.locations = locations.into_iter().collect();
        // TODO: add an assert to check that locations.len() == nodes.len() + 1; this is currently
        // not possible because the true branch of an IfElse block when there is a false branch
        // will not have location for the end token appended at construction time. this location
        // is appended via `add_final_location()` method.
        self
    }

    // STATE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Adds the provided location to the end of location list.
    ///
    /// It is expected that prior to calling this method the number of nodes and locations
    /// contained in this code body is the same. Thus, after calling this method there will be one
    /// more location than node. This is because locations should map `1:1` to their nodes, except
    /// for the block termination that is always the last location.
    ///
    /// # Panics
    /// Panics if the final location has been added previously.
    pub fn add_final_location(&mut self, location: SourceLocation) {
        assert_eq!(self.locations.len(), self.nodes.len());
        self.locations.push(location);
    }

    /// Removes source location information from this code body.
    pub fn clear_locations(&mut self) {
        self.locations.clear();
    }

    // SERIALIZATION / DESERIALIZATION
    // --------------------------------------------------------------------------------------------

    /// Loads the [SourceLocation] from the `source`.
    ///
    /// The `source` is expected to provide a locations count equal to the block nodes count + 1,
    /// having the last element reserved for its `end` node. This way, the locations count is not
    /// expected to be read, as opposed to common vector serialization strategies.
    ///
    /// This implementation intentionally diverges from [Deserializable] so locations can be
    /// optionally stored.
    pub fn load_source_locations<R: ByteReader>(
        &mut self,
        source: &mut R,
    ) -> Result<(), DeserializationError> {
        self.locations = (0..=self.nodes.len())
            .map(|_| SourceLocation::read_from(source))
            .collect::<Result<_, _>>()?;
        Ok(())
    }

    /// Writes the [SourceLocation] into `target`.
    ///
    /// The locations will be written directly, without storing the locations count. This is the
    /// counterpart of [CodeBody::load_source_locations].
    ///
    /// This implementation intentionally diverges from [Serializable] so locations can be
    /// optionally stored.
    pub fn write_source_locations<W: ByteWriter>(&self, target: &mut W) {
        self.locations.iter().for_each(|l| l.write_into(target));
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns the [Node] sequence.
    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }

    /// Returns the [SourceLocations] bound to the nodes of this body structure.
    pub fn source_locations(&self) -> &[SourceLocation] {
        &self.locations
    }

    /// Returns true if this code body contain source location information.
    pub fn has_locations(&self) -> bool {
        !self.locations.is_empty()
    }

    // DESTRUCTURING
    // --------------------------------------------------------------------------------------------

    /// Returns the internal parts of this code body.
    pub fn into_parts(self) -> (Vec<Node>, Vec<SourceLocation>) {
        (self.nodes, self.locations)
    }
}

impl<'a> IntoIterator for &'a CodeBody {
    type Item = (&'a Node, &'a SourceLocation);
    type IntoIter = iter::Zip<slice::Iter<'a, Node>, slice::Iter<'a, SourceLocation>>;

    fn into_iter(self) -> Self::IntoIter {
        self.nodes.iter().zip(self.locations.iter())
    }
}

impl FromIterator<Node> for CodeBody {
    fn from_iter<T: IntoIterator<Item = Node>>(nodes: T) -> Self {
        Self {
            nodes: nodes.into_iter().collect(),
            locations: Vec::new(),
        }
    }
}

impl FromIterator<(Node, SourceLocation)> for CodeBody {
    fn from_iter<T: IntoIterator<Item = (Node, SourceLocation)>>(nodes: T) -> Self {
        let (nodes, locations) = nodes.into_iter().unzip();
        Self { nodes, locations }
    }
}

impl PartialEq for CodeBody {
    fn eq(&self, other: &Self) -> bool {
        // TODO deserialized node will not restore location, but equality must hold
        let nodes = self.nodes == other.nodes;
        let locations = self.locations == other.locations;
        let left_empty = self.locations.is_empty();
        let right_empty = other.locations.is_empty();
        nodes && (locations || left_empty || right_empty)
    }
}
