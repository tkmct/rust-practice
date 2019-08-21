extern crate bit_vec;
use bit_vec::BitVec;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};


pub struct BloomFilter {
    bits: BitVec,
}


impl BloomFilter {
    pub fn new() -> Self {
        BloomFilter {
            bits: BitVec::from_elem(64, false)
        }
    }

    pub fn add<T: Hash>(&mut self, elm: &T) {
        let mut hasher = DefaultHasher::new();
        elm.hash(&mut hasher);
        let hash = hasher.finish().to_be_bytes();
        let bits = BitVec::from_bytes(&hash);

        self.bits.union(&bits);
    }

    pub fn check<T: Hash>(&self, elm: &T) -> bool {
        let mut hasher = DefaultHasher::new();
        elm.hash(&mut hasher);
        let hash = hasher.finish().to_be_bytes();
        let bits = BitVec::from_bytes(&hash);

        let mut res = self.bits.clone();
        !res.union(&bits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check_inclusion() {
        let mut filter = BloomFilter::new();
        filter.add(&12);
        filter.add(&13);
        filter.add(&14);
        filter.add(&15);
        assert!(filter.check(&12));
    }

    #[test]
    fn test_check_exclusion() {
        let mut filter = BloomFilter::new();
        filter.add(&12);
        filter.add(&22);
        filter.add(&32);
        assert!(!filter.check(&42));
    }
}

