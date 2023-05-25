//! Fixed-sized sequence of bits.

#![deny(rust_2018_idioms)]

#[cfg(test)]
mod tests;

use std::fmt::Debug;

use num_traits::{PrimInt, ToPrimitive};

pub struct Bitset<const N: usize, T = usize> {
    data: [T; N],
}

impl<const N: usize, T> Bitset<N, T>
where
    T: PrimInt,
{
    const BITS: usize = std::mem::size_of::<T>() * u8::BITS as usize;

    #[inline]
    fn index_offset(&self, pos: usize) -> (usize, usize) {
        let index = pos / Self::BITS;
        let bit_offset = pos & (Self::BITS - 1);

        (index, bit_offset)
    }

    /// Set all bits to false.
    pub fn reset(&mut self) {
        self.data.fill(T::zero())
    }

    pub fn set(&mut self, pos: usize, value: bool) {
        let (index, bit_offset) = self.index_offset(pos);

        let element = self.data[index];

        if value {
            self.data[index] = element.bitor(T::one() << bit_offset);
        } else {
            self.data[index] = element.bitand(!(T::one() << bit_offset));
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns `true` if the bitset length is 0.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Number of bits in the set.
    pub fn bit_len(&self) -> usize {
        self.data.len() * Self::BITS
    }

    /// Iterator over the bits, returned as `1` or `0`.
    pub fn bits(&self) -> BitIter<'_, N, T> {
        BitIter {
            bitset: self,
            position: 0,
        }
    }

    /// Sets all `true` bits to `false`, and all `false` bits to `true`.
    pub fn flip(&mut self) {
        for element in &mut self.data {
            *element = element.not();
        }
    }

    /// Returns `true` if any bits are set to true.
    pub fn any(&self) -> bool {
        self.data.iter().any(|el| el.gt(&T::zero()))
    }

    /// Returns `true` if all bits are set to true.
    pub fn all(&self) -> bool {
        self.data.iter().all(|el| el.eq(&T::max_value()))
    }

    /// Returns `true` if no bits are set to true.
    pub fn none(&self) -> bool {
        self.data.iter().all(|el| el.eq(&T::zero()))
    }

    /// Retrieve the underlying array storage.
    pub fn as_array(&self) -> &[T; N] {
        &self.data
    }
}

impl<const N: usize, T> Bitset<N, T>
where
    T: PrimInt + Default,
{
    #[inline]
    pub fn new() -> Self {
        assert!(Self::BITS.is_power_of_two());
        Self {
            data: [Default::default(); N],
        }
    }
}

impl<const N: usize, T> Bitset<N, T>
where
    T: PrimInt + ToPrimitive,
{
    #[inline]
    pub fn get_bit(&self, pos: usize) -> u8 {
        let (index, bit_offset) = self.index_offset(pos);

        let element = self.data[index];

        element
            .rotate_right(bit_offset as u32)
            .bitand(T::one())
            .to_u8()
            .unwrap_or_default()
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

impl<const N: usize, T> Debug for Bitset<N, T>
where
    T: PrimInt + Debug + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.bits()).finish()
    }
}

/// Iterator producing the bits as either a `0` or `1`.
pub struct BitIter<'a, const N: usize, T: 'static> {
    bitset: &'a Bitset<N, T>,
    position: usize,
}

impl<'a, const N: usize, T> Iterator for BitIter<'a, N, T>
where
    T: PrimInt + 'static,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position < self.bitset.bit_len() {
            let bit = self.bitset.get_bit(self.position);
            self.position += 1;
            Some(bit)
        } else {
            None
        }
    }
}
