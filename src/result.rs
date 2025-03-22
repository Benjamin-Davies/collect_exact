use crate::FromIteratorExact;

struct State<I, E> {
    iter: I,
    error: Option<E>,
}

impl<A, E, V: FromIteratorExact<A>> FromIteratorExact<Result<A, E>> for Result<V, E> {
    fn from_iter_exact<I: IntoIterator<Item = Result<A, E>>>(
        iter: I,
    ) -> Result<Self, crate::Error> {
        let mut state = State {
            iter: iter.into_iter(),
            error: None,
        };

        let result = V::from_iter_exact(&mut state);

        if let Some(error) = state.error {
            Ok(Err(error))
        } else {
            result.map(Ok)
        }
    }

    fn from_iter_exact_prefix<I: IntoIterator<Item = Result<A, E>>>(
        iter: I,
    ) -> Result<Self, crate::PrefixError> {
        let mut state = State {
            iter: iter.into_iter(),
            error: None,
        };

        let result = V::from_iter_exact_prefix(&mut state);

        if let Some(error) = state.error {
            Ok(Err(error))
        } else {
            result.map(Ok)
        }
    }
}

impl<A, E, I: Iterator<Item = Result<A, E>>> Iterator for State<I, E> {
    type Item = A;

    fn next(&mut self) -> Option<Self::Item> {
        if self.error.is_some() {
            return None;
        }

        match self.iter.next() {
            Some(Ok(item)) => Some(item),
            Some(Err(error)) => {
                self.error = Some(error);
                None
            }
            None => None,
        }
    }
}
