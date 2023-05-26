//! Fixed-sized sequence of bits.
//!
//! ```
//! use bitset::Bitset;
//!
//! let mut a = Bitset::<10, u32>::new();
//!
//! // set bits
//! a.set(0, true);
//! a.set(1, true);
//! a.set(2, true);
//!
//! // unset a bit
//! a.set(1, false);
//!
//! // iterate over the bits as 1 or 0
//! for bit in a.bits() {
//!     print!("{bit}");
//! }
//!
//! assert!(a.any());   // check if any bits are 1
//! assert!(!a.all());  // check if all bits are 1
//! assert!(!a.none()); // check if no bits are 1
//!
//! // flip the bits using bitwise not
//! a.flip();
//! ```

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

    /// Returns the number of bits of one underlying elements.
    ///
    /// ```
    /// # use bitset::Bitset;
    /// assert_eq!(Bitset::<1, u8>::bit_size(), 8);
    /// assert_eq!(Bitset::<1, u16>::bit_size(), 16);
    /// assert_eq!(Bitset::<1, u32>::bit_size(), 32);
    /// assert_eq!(Bitset::<1, u64>::bit_size(), 64);
    /// ```
    pub const fn bit_size() -> usize {
        Self::BITS
    }

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

    pub fn get(&self, pos: usize) -> bool {
        let (index, bit_offset) = self.index_offset(pos);
        let element = self.data[index];
        element
            .rotate_right(bit_offset as u32)
            .bitand(T::one())
            .eq(&T::one())
    }

    /// Returns the number of elements in the underlying array.
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

    /// Iterator over the bits, returned as `true` or `false`.
    pub fn iter(&self) -> Iter<'_, N, T> {
        Iter {
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

/// Iterator producing the bits as either a `false` or `true`.
pub struct Iter<'a, const N: usize, T: 'static> {
    bitset: &'a Bitset<N, T>,
    position: usize,
}

impl<'a, const N: usize, T> Iterator for Iter<'a, N, T>
where
    T: PrimInt + 'static,
{
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position < self.bitset.bit_len() {
            let bit = self.bitset.get(self.position);
            self.position += 1;
            Some(bit)
        } else {
            None
        }
    }
}
