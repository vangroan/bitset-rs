//! Fixed-sized sequence of bits.

#![deny(rust_2018_idioms)]

#[cfg(test)]
mod tests;

use num_traits::PrimInt;

pub struct Bitset<const N: usize, T = usize> {
    data: [T; N],
}

impl<const N: usize, T> Bitset<N, T>
where
    T: PrimInt + Default,
{
    const BITS: usize = std::mem::size_of::<T>() * u8::BITS as usize;

    #[inline]
    pub fn new() -> Self {
        assert!(Self::BITS.is_power_of_two());
        Self {
            data: [Default::default(); N],
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn set(&mut self, pos: usize, value: bool) {
        let index = pos / Self::BITS;
        let bit_offset = pos & (Self::BITS - 1);

        let element = self.data[index];

        if value {
            self.data[index] = element.bitor(T::one() << bit_offset);
        } else {
            self.data[index] = element.bitand(!(T::one() << bit_offset));
        }
    }

    pub fn clear(&mut self) {
        self.data.fill(T::default())
    }

    /// Retrieve the underlying array storage.
    pub fn as_array(&self) -> &[T; N] {
        &self.data
    }
}

impl<const N: usize, T> Default for Bitset<N, T>
where
    T: PrimInt + Default,
{
    fn default() -> Self {
        Self::new()
    }
}