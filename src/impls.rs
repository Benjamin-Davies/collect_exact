use std::mem::MaybeUninit;

pub(crate) fn collect_exact<T, const N: usize>(
    mut iter: impl Iterator<Item = T>,
) -> Result<[T; N], crate::Error> {
    let array = collect_exact_prefix(&mut iter)?;

    if iter.next().is_some() {
        return Err(crate::Error::TooManyItems);
    }

    Ok(array)
}

pub(crate) fn collect_exact_prefix<T, const N: usize>(
    mut iter: impl Iterator<Item = T>,
) -> Result<[T; N], crate::PrefixError> {
    let mut array = MaybeUninit::<[T; N]>::uninit();

    let start_ptr = array.as_mut_ptr().cast::<T>();
    for i in 0..N {
        if let Some(item) = iter.next() {
            // SAFETY: We have ensured that `i` is less than `N`.
            unsafe { start_ptr.add(i).write(item) };
        } else {
            // Drop any items we have initialized.
            for j in 0..i {
                // SAFETY: All elements up to `i` have been initialized.
                unsafe { start_ptr.add(j).drop_in_place() };
            }

            return Err(crate::PrefixError);
        }
    }

    // SAFETY: If we reach this point, then all elements have been initialized.
    let array = unsafe { array.assume_init() };

    Ok(array)
}
