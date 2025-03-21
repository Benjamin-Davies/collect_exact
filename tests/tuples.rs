use collect_exact::{CollectExact, Error};

#[test]
fn test_into_empty_tuple() {
    let iter = [0u8; 0].into_iter();
    let result = iter.collect_exact::<()>();
    assert_eq!(result, Ok(()));
}

#[test]
fn test_too_many_items_into_empty_tuple() {
    let iter = [42].into_iter();
    let result = iter.collect_exact::<()>();
    assert_eq!(result, Err(Error::TooManyItems));
}

#[test]
fn test_into_singleton_tuple() {
    let iter = [42].into_iter();
    let result = iter.collect_exact::<(u8,)>();
    assert_eq!(result, Ok((42,)));
}

#[test]
fn test_into_pair() {
    let iter = [42, 1].into_iter();
    let result = iter.collect_exact::<(u8, u8)>();
    assert_eq!(result, Ok((42, 1)));
}

#[test]
fn test_into_twelve_tuple() {
    let iter = [42; 12].into_iter();
    let result = iter.collect_exact::<(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)>();
    assert_eq!(result, Ok((42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42)));
}
