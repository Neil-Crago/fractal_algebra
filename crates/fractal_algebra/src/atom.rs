//! atom module
use crate::resonance::SemanticUnit;
use crate::traits::Fractal;
use crate::traits::SemanticEq;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq)]
pub enum AtomError {
    EmptyTags,
    InvalidMetadata(String),
    TagSetError(TagSetError),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Metadata {
    pub domain: String,
    pub description: Option<String>,
}

impl Metadata {
    pub fn validate(&self) -> Result<(), AtomError> {
        if self.domain.trim().is_empty() {
            return Err(AtomError::InvalidMetadata("Empty domain".into()));
        }
        Ok(())
    }
}

impl Default for Metadata {
    fn default() -> Self {
        Metadata {
            domain: "Unknown".into(),
            description: None,
        }
    }
}

impl AsRef<Metadata> for Metadata {
    fn as_ref(&self) -> &Metadata {
        self
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FractalAtom<T>
where
    T: Clone + Eq + Hash,
{
    pub value: T,
    pub tags: TagSet,
    pub metadata: Metadata,
}

impl<T> FractalAtom<T>
where
    T: Clone + Eq + Hash,
{
    /// Creates a new FractalAtom, enforcing that `tags` is non-empty
    /// and metadata passes validation rules.
    ///
    /// # Errors
    /// Returns `AtomError` if:
    /// - `tags` is empty
    /// - `metadata` fails domain checks
    pub fn new(value: T, tags: TagSet, metadata: Metadata) -> Result<Self, AtomError> {
        if tags.is_empty() {
            return Err(AtomError::EmptyTags);
        }

        // Placeholder for metadata validation
        // metadata.validate()?; // We'll define this next
        Ok(Self {
            value,
            tags,
            metadata,
        })
    }

    /// Immutable access to the atom’s value.
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Immutable iterator over sorted tags (canonical order).
    pub fn tags(&self) -> impl Iterator<Item = &String> {
        self.tags.iter()
    }

    /// Immutable access to validated metadata.
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

/// Error cases when constructing or manipulating a TagSet.
#[derive(Debug, PartialEq, Eq)]
pub enum TagSetError {
    EmptyCollection,
    EmptyTag(String),
    DuplicateTag(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TagSet {
    tags: Vec<String>,
}

impl TagSet {

    pub fn new<I: IntoIterator<Item = impl Into<String>>>(
    raw_tags: I,
) -> Result<Self, TagSetError> {
    let mut seen = HashSet::new();
    let mut tags = Vec::new();

    for raw in raw_tags {
        let tag = raw.into().trim().to_string();

        if tag.is_empty() {
            return Err(TagSetError::EmptyTag(tag));
        }

        if !seen.insert(tag.clone()) {
            return Err(TagSetError::DuplicateTag(tag));
        }

        tags.push(tag);
    }

    // THIS IS THE FIX:
    // If the loop finished and we still have no tags,
    // it means the input collection was empty.
    if tags.is_empty() {
        return Err(TagSetError::EmptyCollection);
    }

    tags.sort(); // Optional: for deterministic ordering

    Ok(Self { tags })
}


    /// Returns true if there are no tags.
    pub fn is_empty(&self) -> bool {
        self.tags.is_empty()
    }

    /// Returns the number of tags.
    pub fn len(&self) -> usize {
        self.tags.len()
    }

    /// Returns an iterator over tags in lexical order.
    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.tags.iter()
    }

    /// Checks membership of a tag.
    pub fn contains(&self, tag: &str) -> bool {
        self.tags.binary_search(&tag.to_string()).is_ok()
    }
}

impl Default for TagSet {
    fn default() -> Self {
        TagSet::new(vec!["untagged"]).unwrap()
    }
}

impl From<Vec<String>> for TagSet {
    fn from(raw_tags: Vec<String>) -> Self {
        TagSet::new(raw_tags).unwrap_or_default()
    }
}

impl IntoIterator for TagSet {
    type Item = String;
    type IntoIter = std::vec::IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        self.tags.into_iter()
    }
}

impl<T> SemanticEq for FractalAtom<T>
where
    T: Clone + Eq + Hash + Debug,
{
    fn semantic_eq(&self, other: &Self) -> bool {
        self.metadata.description == other.metadata.description
            && self.tags.semantic_eq(&other.tags)
    }
}

impl SemanticEq for TagSet {
    fn semantic_eq(&self, other: &Self) -> bool {
        self.tags == other.tags // or something more nuanced
    }
}

impl SemanticEq for SemanticUnit {
    fn semantic_eq(&self, other: &Self) -> bool {
        self.label == other.label
            && self.depth == other.depth
            && self.phase == other.phase
            && self.fractal.semantic_eq(&other.fractal)
    }
}

impl<T: SemanticEq + ?Sized> SemanticEq for Box<T> {
    fn semantic_eq(&self, other: &Self) -> bool {
        (**self).semantic_eq(&**other)
    }
}

impl SemanticEq for dyn Fractal {
    fn semantic_eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}
