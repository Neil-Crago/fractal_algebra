//! Defines the `FractalAtom`, a fundamental building block representing a value with semantic context.
//!
//! An `Atom` encapsulates a generic `value`, a set of descriptive `tags`, and structured `metadata`.
//! This module provides strict validation rules to ensure that every created atom is well-formed,
//! for instance by requiring non-empty tags and valid metadata.

use crate::resonance::SemanticUnit;
use crate::traits::{Fractal, SemanticEq};
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

/// Represents errors that can occur during `FractalAtom` or `TagSet` construction.
#[derive(Debug, PartialEq, Eq)]
pub enum AtomError {
    /// Indicates that a `FractalAtom` was created with an empty `TagSet`.
    EmptyTags,
    /// Indicates that the provided `Metadata` failed validation.
    InvalidMetadata(String),
    /// Encapsulates a specific error from `TagSet` construction.
    TagSetError(TagSetError),
}

/// Structured metadata providing context for a `FractalAtom`.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Metadata {
    /// The primary domain or category of the atom (e.g., "physics", "linguistics").
    pub domain: String,
    /// An optional, human-readable description.
    pub description: Option<String>,
}

impl Metadata {
    /// Validates the metadata to ensure it meets structural requirements.
    pub fn validate(&self) -> Result<(), AtomError> {
        if self.domain.trim().is_empty() {
            return Err(AtomError::InvalidMetadata("Domain cannot be empty".into()));
        }
        Ok(())
    }
}

impl AsRef<Metadata> for Metadata {
    fn as_ref(&self) -> &Metadata {
        self
    }
}

/// A wrapper around a value `T` that adds semantic context through tags and metadata.
///
/// Every `FractalAtom` is guaranteed to have at least one tag and valid metadata,
/// making it a robust and context-aware data structure.
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
    /// Creates a new `FractalAtom` after validating its components.
    ///
    /// # Errors
    /// Returns `AtomError` if `tags` is empty or `metadata` is invalid.
    pub fn new(value: T, tags: TagSet, metadata: Metadata) -> Result<Self, AtomError> {
        if tags.is_empty() {
            return Err(AtomError::EmptyTags);
        }
        metadata.validate()?;
        Ok(Self { value, tags, metadata })
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

/// Defines specific error cases that can occur when constructing or manipulating a `TagSet`.
#[derive(Debug, PartialEq, Eq)]
pub enum TagSetError {
    /// The initial collection provided to create the `TagSet` was empty.
    EmptyCollection,
    /// A tag was an empty or whitespace-only string.
    EmptyTag(String),
    /// A duplicate tag was found in the initial collection.
    DuplicateTag(String),
}

/// A validated, sorted, and unique set of string tags.
///
/// This struct guarantees that it is never empty, contains no empty strings,
/// and holds no duplicate tags. Tags are stored in a lexicographically sorted order.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TagSet {
    // Internally, tags are kept sorted and unique.
    tags: Vec<String>,
}

impl TagSet {
    /// Creates a new `TagSet` from an iterator of strings.
    ///
    /// This constructor enforces that the input is not empty, contains no empty strings,
    /// and has no duplicate tags. The resulting tags are sorted.
    ///
    /// # Errors
    /// Returns `TagSetError` if any validation rule is violated.
    pub fn new<I: IntoIterator<Item = impl Into<String>>>(
        raw_tags: I,
    ) -> Result<Self, TagSetError> {
        let mut seen = HashSet::new();
        let mut tags: Vec<String> = Vec::new();

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

        if tags.is_empty() {
            return Err(TagSetError::EmptyCollection);
        }

        tags.sort_unstable(); // Use unstable sort as order of equal elements doesn't matter.
        Ok(Self { tags })
    }

    /// Returns true if there are no tags.
    /// Note: Due to constructor validation, this should always return `false` for a valid `TagSet`.
    pub fn is_empty(&self) -> bool {
        self.tags.is_empty()
    }

    /// Returns the number of tags in the set.
    pub fn len(&self) -> usize {
        self.tags.len()
    }

    /// Returns a sorted iterator over the tags.
    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.tags.iter()
    }

    /// Checks if a given tag is present in the set.
    /// This operation is efficient due to the sorted nature of the internal storage.
    pub fn contains(&self, tag: &str) -> bool {
        self.tags.binary_search(&tag.to_string()).is_ok()
    }
}

impl Default for TagSet {
    /// Creates a default `TagSet` containing only the tag "untagged".
    fn default() -> Self {
        TagSet::new(vec!["untagged"]).unwrap()
    }
}

impl From<Vec<String>> for TagSet {
    /// Attempts to create a `TagSet` from a `Vec<String>`.
    ///
    /// If creation fails (e.g., due to empty input), it falls back to the `default()` implementation.
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

// ... Trait implementations for SemanticEq ...

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
        self.tags == other.tags
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