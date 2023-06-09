use super::{
    ByteReader, ByteWriter, Deserializable, DeserializationError, Node, Serializable,
    SourceLocation,
};
use core::{iter, slice};

// CODE BODY
// ================================================================================================

/// A parsed code container to bind a contiguous sequence of [Node] to their optional
/// [SourceLocation].
///
/// Will yield an iterator of each [Node] with its respective [SourceLocation]. The iterator will
/// be empty if the [SourceLocation] isn't provided.
#[derive(Clone, Default, Eq, Debug)]
pub struct CodeBody {
    nodes: Vec<Node>,
    locations: Vec<SourceLocation>,
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

impl CodeBody {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------

    /// Creates a new instance with the provided `nodes`.
    pub fn new<N>(nodes: N) -> Self
    where
        N: IntoIterator<Item = Node>,
    {
        Self {
            nodes: nodes.into_iter().collect(),
            locations: Vec::new(),
        }
    }

    /// Binds [SourceLocation] to their respective [Node].
    ///
    /// It is expected to have the `locations` length equal to the `self.nodes` length.
    pub fn with_source_locations<L>(mut self, locations: L) -> Self
    where
        L: IntoIterator<Item = SourceLocation>,
    {
        self.locations = locations.into_iter().collect();
        self
    }

    // STATE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Pushes the provided location to the structure.
    ///
    /// Locations are expected to map `1:1` to their nodes; except for the block termination that
    /// is always the last location.
    pub fn push_location(&mut self, location: SourceLocation) {
        self.locations.push(location);
    }

    /// Replaces the source locations for this instance.
    pub fn replace_locations(&mut self, locations: Vec<SourceLocation>) {
        self.locations = locations;
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

    // DESTRUCTURING
    // --------------------------------------------------------------------------------------------

    /// Returns the internal parts of this parsed code.
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
