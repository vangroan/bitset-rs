use super::Bitset;

#[test]
fn test_bit_size() {
    assert_eq!(Bitset::<16, u8>::BITS, 8);
    assert_eq!(Bitset::<16, u16>::BITS, 16);
    assert_eq!(Bitset::<16, u32>::BITS, 32);
    assert_eq!(Bitset::<16, u64>::BITS, 64);
}

#[test]
fn test_set() {
    let mut bitset = Bitset::<2, u32>::new();

    bitset.set(0, true);
    assert_eq!(bitset.as_array(), &[1, 0]);
    bitset.set(1, true);
    assert_eq!(bitset.as_array(), &[3, 0]);
    bitset.set(32, true);
    assert_eq!(bitset.as_array(), &[3, 1]);
    bitset.set(0, false);
    assert_eq!(bitset.as_array(), &[2, 1]);

    bitset.clear();
    assert_eq!(bitset.as_array(), &[0, 0]);

    bitset.set(33, true);
    assert_eq!(bitset.as_array(), &[0, 2]);
}
