pub fn new_data(width: usize) -> Vec<bool> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    std::iter::repeat_with(|| rng.gen()).take(width).collect()
}

pub mod bv_utils {
    use bv::*;

    pub fn new(width: usize) -> BitVec {
        BitVec::with_capacity(width as u64)
    }

    pub fn new_from(bits: &[bool]) -> BitVec {
        let mut bit_vec = new(bits.len());
        for bit in bits {
            bit_vec.push(*bit)
        }
        bit_vec
    }

    #[inline]
    pub fn count_ones(bit_vec: &BitVec) -> usize {
        let mut count = 0;
        for i in 0..bit_vec.len() {
            count += if bit_vec[i] { 1 } else { 0 };
        }
        count
    }

    #[inline]
    pub fn iter_ones(bit_vec: &BitVec) -> Option<u64> {
        (0..bit_vec.len())
            .filter_map(|i| bit_vec[i].then_some(i as usize))
            .max()
            .map(|x| x as u64)
    }
}

pub mod bitvec_utils {
    use bitvec::vec::BitVec;

    pub fn new(width: usize) -> BitVec {
        BitVec::with_capacity(width)
    }

    pub fn new_from(bits: &[bool]) -> BitVec {
        let mut bit_vec = new(bits.len());
        for bit in bits {
            bit_vec.push(*bit)
        }
        bit_vec
    }

    #[inline]
    pub fn count_ones(bit_vec: &BitVec) -> usize {
        bit_vec.count_ones()
    }

    #[inline]
    pub fn iter_ones(bit_vec: &BitVec) -> Option<u64> {
        Iterator::max(bit_vec.iter_ones()).map(|x| x as u64)
    }
}

pub mod bit_vec_utils {
    use bit_vec::*;

    pub fn new(width: usize) -> BitVec {
        BitVec::with_capacity(width)
    }

    pub fn new_from(bits: &[bool]) -> BitVec {
        let mut bit_vec = new(bits.len());
        for bit in bits {
            bit_vec.push(*bit)
        }
        bit_vec
    }

    #[inline]
    pub fn count_ones(bit_vec: &BitVec) -> usize {
        bit_vec.blocks().map(BitBlock::count_ones).sum()
    }

    #[inline]
    pub fn iter_ones(bit_vec: &BitVec) -> Option<u64> {
        bit_vec
            .iter()
            .enumerate()
            .filter_map(|(i, bit)| bit.then_some(i))
            .max()
            .map(|x| x as u64)
    }
}

pub mod hash_set_utils {
    use std::collections::HashSet;

    pub fn new_from(bits: &[bool]) -> HashSet<u64> {
        let mut hash_set = HashSet::new();
        bits.iter()
            .enumerate()
            .filter(|(_, bit)| **bit)
            .for_each(|(i, _)| {
                let did_insert = hash_set.insert(i as u64);
                assert!(did_insert);
            });
        hash_set
    }

    #[inline]
    pub fn count_ones(hash_set: &HashSet<u64>) -> usize {
        // this impl could be hash_set.len(), but that's not really useful for a benchmark
        hash_set.iter().count()
    }

    #[inline]
    pub fn iter_ones(hash_set: &HashSet<u64>) -> Option<u64> {
        hash_set.iter().max().cloned()
    }
}

pub mod int_set_utils {
    use solana_nohash_hasher::IntSet;

    pub fn new_from(bits: &[bool]) -> IntSet<u64> {
        let mut int_set = IntSet::default();
        bits.iter()
            .enumerate()
            .filter(|(_, bit)| **bit)
            .for_each(|(i, _)| {
                let did_insert = int_set.insert(i as u64);
                assert!(did_insert);
            });
        int_set
    }

    #[inline]
    pub fn count_ones(int_set: &IntSet<u64>) -> usize {
        // this impl could be int_set.len(), but that's not really useful for a benchmark
        int_set.iter().count()
    }

    #[inline]
    pub fn iter_ones(hash_set: &IntSet<u64>) -> Option<u64> {
        hash_set.iter().max().cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_ones() {
        let data = new_data(1024);

        let bv_result = {
            let bit_vec = bv_utils::new_from(data.as_slice());
            bv_utils::count_ones(&bit_vec)
        };
        let bitvec_result = {
            let bit_vec = bitvec_utils::new_from(data.as_slice());
            bitvec_utils::count_ones(&bit_vec)
        };
        let bit_vec_result = {
            let bit_vec = bit_vec_utils::new_from(data.as_slice());
            bit_vec_utils::count_ones(&bit_vec)
        };
        let hash_set_result = {
            let hash_set = hash_set_utils::new_from(data.as_slice());
            hash_set_utils::count_ones(&hash_set)
        };
        let int_set_result = {
            let int_set = int_set_utils::new_from(data.as_slice());
            int_set_utils::count_ones(&int_set)
        };

        assert_eq!(bv_result, bitvec_result);
        assert_eq!(bv_result, bit_vec_result);
        assert_eq!(bv_result, hash_set_result);
        assert_eq!(bv_result, int_set_result);
    }

    #[test]
    fn test_iter_ones() {
        let data = new_data(1024);

        let bv_result = {
            let bit_vec = bv_utils::new_from(data.as_slice());
            bv_utils::iter_ones(&bit_vec)
        };
        let bitvec_result = {
            let bit_vec = bitvec_utils::new_from(data.as_slice());
            bitvec_utils::iter_ones(&bit_vec)
        };
        let bit_vec_result = {
            let bit_vec = bit_vec_utils::new_from(data.as_slice());
            bit_vec_utils::iter_ones(&bit_vec)
        };
        let hash_set_result = {
            let hash_set = hash_set_utils::new_from(data.as_slice());
            hash_set_utils::iter_ones(&hash_set)
        };
        let int_set_result = {
            let int_set = int_set_utils::new_from(data.as_slice());
            int_set_utils::iter_ones(&int_set)
        };

        assert_eq!(bv_result, bitvec_result);
        assert_eq!(bv_result, bit_vec_result);
        assert_eq!(bv_result, hash_set_result);
        assert_eq!(bv_result, int_set_result);
    }
}
