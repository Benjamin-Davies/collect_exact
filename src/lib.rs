#![warn(missing_docs)]
#![warn(clippy::pedantic)]
// TODO: write docs
#![allow(clippy::missing_errors_doc)]
#![allow(missing_docs)]

mod error;
mod impls;
mod result;
mod tuples;

pub use error::Error;

pub trait CollectExact {
    type Item;

    fn collect_exact<B: FromIteratorExact<Self::Item>>(self) -> Result<B, Error>
    where
        Self: Sized;

    fn collect_exact_prefix<B: FromIteratorExact<Self::Item>>(self) -> Result<B, Error>
    where
        Self: Sized;
}

pub trait FromIteratorExact<T>: Sized {
    fn from_iter_exact<I: IntoIterator<Item = T>>(iter: I) -> Result<Self, Error>;

    fn from_iter_exact_prefix<I: IntoIterator<Item = T>>(iter: I) -> Result<Self, Error>;
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

    fn collect_exact_prefix<B: FromIteratorExact<Self::Item>>(self) -> Result<B, Error>
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

    fn from_iter_exact_prefix<I: IntoIterator<Item = T>>(iter: I) -> Result<Self, Error> {
        impls::collect_exact_prefix(iter.into_iter())
    }
}
