# Bitset

![Checks Workflow](https://github.com/vangroan/bitset-rs/actions/workflows/checks.yaml/badge.svg?branch=main&event=push)

A fixed-sized sequence of bits.

```rust
use bitset::Bitset;

let mut a = Bitset::<10, u32>::new();

// set bits
a.set(0, true);
a.set(1, true);
a.set(2, true);

// unset a bit
a.set(1, false);

// iterate over the bits as 1 or 0
for bit in a.bits() {
    print!("{bit}");
}

assert!(a.any());   // check if any bits are 1
assert!(!a.all());  // check if all bits are 1
assert!(!a.none()); // check if no bits are 1

// flip the bits using bitwise not
a.flip();
```

## Licence

This library is licensed under the MIT open source license. See [License](LICENSE)
