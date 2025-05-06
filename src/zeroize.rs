use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use zeroize::{DefaultIsZeroes, Zeroize as _};

/// `ZeroizeSlice` is a a wrapper for any `T: AsMut<[Z: DefaultIsZeroes]>` type
/// which implements a `Drop` handler which zeroizes dropped values.
/// The strict bounds ensure that the fast `zeroize` implementation is used on `Drop`.
#[derive(Debug)]
pub(crate) struct ZeroizeSlice<T, Z = u8>
where
    T: AsMut<[Z]>,
    Z: DefaultIsZeroes,
{
    value: T,
    _phantom: PhantomData<Z>,
}

impl<T, Z> ZeroizeSlice<T, Z>
where
    T: AsMut<[Z]>,
    Z: DefaultIsZeroes,
{
    #[inline(always)]
    pub(crate) fn new(value: T) -> Self {
        Self {
            value,
            _phantom: PhantomData,
        }
    }
}

impl<T, Z> Deref for ZeroizeSlice<T, Z>
where
    T: AsMut<[Z]>,
    Z: DefaultIsZeroes,
{
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T, Z> DerefMut for ZeroizeSlice<T, Z>
where
    T: AsMut<[Z]>,
    Z: DefaultIsZeroes,
{
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T, Z> AsRef<T> for ZeroizeSlice<T, Z>
where
    T: AsMut<[Z]>,
    Z: DefaultIsZeroes,
{
    #[inline(always)]
    fn as_ref(&self) -> &T {
        &self.value
    }
}

impl<T, Z> AsMut<T> for ZeroizeSlice<T, Z>
where
    T: AsMut<[Z]>,
    Z: DefaultIsZeroes,
{
    #[inline(always)]
    fn as_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<Z> ZeroizeSlice<Vec<Z>, Z>
where
    Z: DefaultIsZeroes,
{
    #[inline(always)]
    pub(crate) fn into_boxed_slice(mut self) -> ZeroizeSlice<Box<[Z]>, Z> {
        let inner = std::mem::replace(&mut self.value, Vec::with_capacity(0));
        ZeroizeSlice::<Box<[Z]>, Z>::new(inner.into_boxed_slice())
    }
}

impl<T, Z> Drop for ZeroizeSlice<T, Z>
where
    T: AsMut<[Z]>,
    Z: DefaultIsZeroes,
{
    fn drop(&mut self) {
        // Using `into_boxed_slice` leads to dropping an empty Vec, so avoid the atomic fence
        // by just not zeroizing it
        if !self.value.as_mut().is_empty() {
            // Iterate in chunks to always fulfill the assertion and avoid the panic in the zeroize function
            for chunk in self.value.as_mut().chunks_mut(isize::MAX as usize) {
                chunk.zeroize();
            }
        }
    }
}
