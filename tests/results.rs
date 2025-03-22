use collect_exact::{CollectExact, Error};

#[test]
fn test_into_result() {
    let iter = [Ok(42); 256].into_iter();
    let result = iter.collect_exact::<Result<[u8; 256], u8>>();
    assert_eq!(result, Ok(Ok([42; 256])));
}

#[test]
fn test_prefix_into_result() {
    let iter = [Ok(42); 256].into_iter();
    let result = iter.collect_exact_prefix::<Result<[u8; 128], u8>>();
    assert_eq!(result, Ok(Ok([42; 128])));
}

#[test]
fn test_error_into_result() {
    let iter = [Ok(42), Err(42)].into_iter();
    let result = iter.collect_exact::<Result<[u8; 2], u8>>();
    assert_eq!(result, Ok(Err(42)));
}

#[test]
fn test_too_few_items_into_result() {
    let iter = [Ok(42); 255].into_iter();
    let result = iter.collect_exact::<Result<[u8; 256], u8>>();
    assert_eq!(result, Err(Error::TooFewItems));
}

#[test]
fn test_stops_after_error_into_result() {
    struct State {
        counter: u8,
    }

    impl Iterator for State {
        type Item = Result<u8, u8>;

        fn next(&mut self) -> Option<Self::Item> {
            let n = self.counter;
            match n {
                0..=41 => {
                    self.counter += 1;
                    Some(Ok(n))
                }
                42 => {
                    self.counter += 1;
                    Some(Err(n))
                }
                _ => panic!(),
            }
        }
    }

    let state = State { counter: 0 };
    let result = state.collect_exact::<Result<[u8; 256], u8>>();
    assert_eq!(result, Ok(Err(42)));
}
