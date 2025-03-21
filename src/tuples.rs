use crate::FromIteratorExact;

// https://stackoverflow.com/a/56700760
macro_rules! replace_expr {
    ($_t:tt $sub:ty) => {
        $sub
    };
}

macro_rules! tuple_impls {
    ( $n:literal; $( $name:ident )* ) => {
        #[allow(non_snake_case)]
        impl<T> FromIteratorExact<T> for ($(replace_expr!(($name) T),)*) {
            fn from_iter_exact<I: IntoIterator<Item = T>>(iter: I) -> Result<Self, crate::Error> {
                <[T; $n]>::from_iter_exact(iter).map(|[ $($name,)* ]| ( $($name,)* ))
            }

            fn from_iter_exact_prefix<I: IntoIterator<Item = T>>(iter: I) -> Result<Self, crate::Error> {
                <[T; $n]>::from_iter_exact_prefix(iter).map(|[ $($name,)* ]| ( $($name,)* ))
            }
        }
    };
}

tuple_impls! { 0; }
tuple_impls! { 1; A }
tuple_impls! { 2; A B }
tuple_impls! { 3; A B C }
tuple_impls! { 4; A B C D }
tuple_impls! { 5; A B C D E }
tuple_impls! { 6; A B C D E F }
tuple_impls! { 7; A B C D E F G }
tuple_impls! { 8; A B C D E F G H }
tuple_impls! { 9; A B C D E F G H I }
tuple_impls! { 10; A B C D E F G H I J }
tuple_impls! { 11; A B C D E F G H I J K }
tuple_impls! { 12; A B C D E F G H I J K L }
