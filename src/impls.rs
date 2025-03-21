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
) -> Result<[T; N], crate::Error> {
    unsafe {
        let mut array = MaybeUninit::<[T; N]>::uninit();

        let start_ptr = array.as_mut_ptr().cast::<T>();
        for i in 0..N {
            if let Some(item) = iter.next() {
                start_ptr.add(i).write(item);
            } else {
                for j in 0..i {
                    start_ptr.add(j).drop_in_place();
                }
                return Err(crate::Error::TooFewItems);
            }
        }

        Ok(array.assume_init())
    }
}
