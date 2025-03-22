use collect_exact::{CollectExact, Error, PrefixError};

#[test]
fn test_into_array() {
    let iter = [42; 256].into_iter();
    let result = iter.collect_exact::<[u8; 256]>();
    assert_eq!(result, Ok([42; 256]));
}

#[test]
fn test_too_few_items_into_array() {
    let iter = [42; 255].into_iter();
    let result = iter.collect_exact::<[u8; 256]>();
    assert_eq!(result, Err(Error::TooFewItems));
}

#[test]
fn test_too_many_items_into_array() {
    let iter = [42; 257].into_iter();
    let result = iter.collect_exact::<[u8; 256]>();
    assert_eq!(result, Err(Error::TooManyItems));
}

#[test]
fn test_into_empty_array() {
    let iter = [].into_iter();
    let result = iter.collect_exact::<[u8; 0]>();
    assert_eq!(result, Ok([]));
}

#[test]
fn test_too_many_items_into_empty_array() {
    let iter = [42].into_iter();
    let result = iter.collect_exact::<[u8; 0]>();
    assert_eq!(result, Err(Error::TooManyItems));
}

#[test]
fn test_into_singleton_array() {
    let iter = [42].into_iter();
    let result = iter.collect_exact::<[u8; 1]>();
    assert_eq!(result, Ok([42]));
}

#[test]
fn test_too_few_items_into_singleton_array() {
    let iter = [].into_iter();
    let result = iter.collect_exact::<[u8; 1]>();
    assert_eq!(result, Err(Error::TooFewItems));
}

#[test]
fn test_too_many_items_into_singleton_array() {
    let iter = [42, 1].into_iter();
    let result = iter.collect_exact::<[u8; 1]>();
    assert_eq!(result, Err(Error::TooManyItems));
}

#[test]
fn test_prefix_into_array() {
    let iter = [42; 256].into_iter();
    let result = iter.collect_exact_prefix::<[u8; 128]>();
    assert_eq!(result, Ok([42; 128]));
}

#[test]
fn test_prefix_too_few_items_into_array() {
    let iter = [42; 127].into_iter();
    let result = iter.collect_exact_prefix::<[u8; 128]>();
    assert_eq!(result, Err(PrefixError));
}
