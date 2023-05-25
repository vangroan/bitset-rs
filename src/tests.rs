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

    bitset.reset();
    assert_eq!(bitset.as_array(), &[0, 0]);

    bitset.set(33, true);
    assert_eq!(bitset.as_array(), &[0, 2]);
}

#[test]
fn test_any() {
    let mut bitset = Bitset::<2, u32>::new();

    assert!(!bitset.any());
    bitset.set(0, true);
    assert!(bitset.any());
}

#[test]
fn test_all() {
    let mut bitset = Bitset::<2, u32>::new();

    assert!(!bitset.all());
    bitset.set(0, true);
    assert!(!bitset.all());

    for i in 0..64 {
        bitset.set(i, true);
    }
    assert!(bitset.all());
}

#[test]
fn test_none() {
    let mut bitset = Bitset::<2, u32>::new();

    assert!(bitset.none());
    bitset.set(0, true);
    assert!(!bitset.none());
}

#[test]
fn test_flip() {
    let mut bitset = Bitset::<2, u32>::new();

    bitset.set(0, true);
    assert_eq!(bitset.as_array(), &[1, 0]);

    bitset.flip();
    assert_eq!(bitset.as_array(), &[0xFFFFFFFE, 0xFFFFFFFF]);
}

#[test]
fn test_bit_iter() {
    let mut bitset = Bitset::<2, u32>::new();

    bitset.set(0, true);
    bitset.set(7, true);
    bitset.set(11, true);
    bitset.set(33, true);
    bitset.set(42, true);

    println!("{bitset:?}");

    let mut count = 0;

    for (i, bit) in bitset.bits().enumerate() {
        match i {
            0 | 7 | 11 | 33 | 42 => assert_eq!(bit, 1, "index: {i}; bit: {bit}"),
            _ => assert_eq!(bit, 0, "index: {i}; bit: {bit}"),
        }

        count += 1;
    }

    assert_eq!(count, 64);
}
