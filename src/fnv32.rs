//! An implementation of the [Fowler–Noll–Vo hash function][chongo].
//!
//! ## About
//!
//! The FNV hash function is a custom `Hasher` implementation that is more
//! efficient for smaller hash keys.
//!
//! [The Rust FAQ states that][faq] while the default `Hasher` implementation,
//! SipHash, is good in many cases, it is notably slower than other algorithms
//! with short keys, such as when you have a map of integers to other values.
//! In cases like these, [FNV is demonstrably faster][graphs].
//!
//! Its disadvantages are that it performs badly on larger inputs, and
//! provides no protection against collision attacks, where a malicious user
//! can craft specific keys designed to slow a hasher down. Thus, it is
//! important to profile your program to ensure that you are using small hash
//! keys, and be certain that your program could not be exposed to malicious
//! inputs (including being a networked server).
//!
//! The Rust compiler itself uses FNV, as it is not worried about
//! denial-of-service attacks, and can assume that its inputs are going to be
//! small—a perfect use case for FNV.
//!
//!
//! ## Using FNV in a `HashMap`
//!
//! The `FnvHashMap` type alias is the easiest way to use the standard library’s
//! `HashMap` with FNV.
//!
//! ```rust
//! use fnv::FnvHashMap;
//!
//! let mut map = FnvHashMap::default();
//! map.insert(1, "one");
//! map.insert(2, "two");
//!
//! map = FnvHashMap::with_capacity_and_hasher(10, Default::default());
//! map.insert(1, "one");
//! map.insert(2, "two");
//! ```
//!
//! Note, the standard library’s `HashMap::new` and `HashMap::with_capacity`
//! are only implemented for the `RandomState` hasher, so using `Default` to
//! get the hasher is the next best option.
//!
//! ## Using FNV in a `HashSet`
//!
//! Similarly, `FnvHashSet` is a type alias for the standard library’s `HashSet`
//! with FNV.
//!
//! ```rust
//! use fnv::FnvHashSet;
//!
//! let mut set = FnvHashSet::default();
//! set.insert(1);
//! set.insert(2);
//!
//! set = FnvHashSet::with_capacity_and_hasher(10, Default::default());
//! set.insert(1);
//! set.insert(2);
//! ```
//!
//! [chongo]: http://www.isthe.com/chongo/tech/comp/fnv/index.html
//! [faq]: https://www.rust-lang.org/en-US/faq.html#why-are-rusts-hashmaps-slow
//! [graphs]: http://cglab.ca/~abeinges/blah/hash-rs/


use std::default::Default;

/// An implementation of the Fowler–Noll–Vo hash function.
///
/// See the [crate documentation](index.html) for more details.
#[allow(missing_copy_implementations)]
pub struct FnvHasher(u32);

impl Default for FnvHasher {

    #[inline]
    fn default() -> FnvHasher {
        FnvHasher(0x811c9dc5)
    }
}

impl FnvHasher {

    #[inline]
    pub fn finish(&self) -> u32 {
        self.0
    }

    #[inline]
    pub fn write(&mut self, bytes: &[u8]) {
        let FnvHasher(mut hash) = *self;

        for byte in bytes.iter() {
            hash = hash ^ (*byte as u32);
            hash = hash.wrapping_mul(0x811c9dc5);
        }

        *self = FnvHasher(hash);
    }
}
