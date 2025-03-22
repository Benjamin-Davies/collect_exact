//! Allows zero-cost collection into exact-size arrays and tuples.
//!
//! ## Usage
//!
//! ```rust
//! use collect_exact::CollectExact;
//!
//! let iter = [1, 2, 3].into_iter();
//! let result = iter.collect_exact::<[u32; 3]>();
//!
//! assert_eq!(result, Ok([1, 2, 3]));
//! ```

#![warn(missing_docs)]
#![warn(clippy::pedantic)]

mod error;
mod impls;
mod result;
mod tuples;

pub use error::{Error, PrefixError};

/// Extension trait for collecting into collections whose length is known at compile time.
pub trait CollectExact {
    /// The type of items in the iterator.
    type Item;

    /// Transforms an iterator into a collection whose length is known at compile time.
    ///
    /// See also [`Iterator::collect`].
    ///
    /// # Errors
    ///
    /// Returns an error if the iterator does not contain the same number of items as the collection.
    fn collect_exact<B: FromIteratorExact<Self::Item>>(self) -> Result<B, Error>
    where
        Self: Sized;

    /// Transforms a prefix of the iterator into a collection whose length is known at compile time.
    /// Drops the iterator once it has collected enough items.
    ///
    /// See also [`CollectExact::collect_exact`].
    ///
    /// # Errors
    ///
    /// Returns an error if the iterator contains fewer items than the collection.
    fn collect_exact_prefix<B: FromIteratorExact<Self::Item>>(self) -> Result<B, PrefixError>
    where
        Self: Sized;
}

/// Conversion from an [`Iterator`] for a collection whose length is known at compile time.
pub trait FromIteratorExact<T>: Sized {
    /// Creates a collection from an iterator.
    ///
    /// See also [`CollectExact::collect_exact`].
    ///
    /// # Errors
    ///
    /// Returns an error if the iterator does not contain the same number of items as the collection.
    fn from_iter_exact<I: IntoIterator<Item = T>>(iter: I) -> Result<Self, Error>;

    /// Creates a collection from a prefix of an iterator.
    ///
    /// See also [`CollectExact::collect_exact_prefix`].
    ///
    /// # Errors
    ///
    /// Returns an error if the iterator contains fewer items than the collection.
    fn from_iter_exact_prefix<I: IntoIterator<Item = T>>(iter: I) -> Result<Self, PrefixError>;
}

impl<Iter> CollectExact for Iter
where
    Iter: Iterator,
{
    type Item = <Self as Iterator>::Item;

    fn collect_exact<B: FromIteratorExact<Self::Item>>(self) -> Result<B, Error>
    where
        Self: Sized,
    {
        FromIteratorExact::from_iter_exact(self)
    }

    fn collect_exact_prefix<B: FromIteratorExact<Self::Item>>(self) -> Result<B, PrefixError>
    where
        Self: Sized,
    {
        FromIteratorExact::from_iter_exact_prefix(self)
    }
}

impl<T, const N: usize> FromIteratorExact<T> for [T; N] {
    fn from_iter_exact<I: IntoIterator<Item = T>>(iter: I) -> Result<Self, Error> {
        impls::collect_exact(iter.into_iter())
    }

    fn from_iter_exact_prefix<I: IntoIterator<Item = T>>(iter: I) -> Result<Self, PrefixError> {
        impls::collect_exact_prefix(iter.into_iter())
    }
}
