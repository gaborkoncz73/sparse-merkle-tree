use crate::h256::H256;
use crate::traits::{Hasher, Value};
use crate::Key;

/// Merge two hashes
/// this function is optimized for ZERO_HASH
/// if one of lhs or rhs is ZERO_HASH, this function just returns the another one
pub fn merge<H: Hasher + Default>(lhs: &H256, rhs: &H256) -> H256 {
    /*if lhs.is_zero() {
        return *rhs;
    } else if rhs.is_zero() {
        return *lhs;
    }*/
    let mut hasher = H::default();
    hasher.write_bytes(lhs.as_slice());
    hasher.write_bytes(rhs.as_slice());
    hasher.finish()
}

/// hash_leaf = hash(prefix | key | value)
/// zero value indicates a key is to be deleted, this function returns zero for zero value
pub fn hash_leaf<H: Hasher + Default, K, V, const N: usize>(key: &K, value: &V) -> H256
where
    K: Key<N>,
    V: Value,
{
    if value.is_zero() {
        let mut hasher = H::default();
        hasher.write_bytes(H256::zero().as_slice());
        return  hasher.finish();
    }
    let mut hasher = H::default();
    hasher.write_bytes(&[1u8]);
    hasher.write_bytes(key.as_slice());
    hasher.write_bytes(value.as_slice());
    hasher.finish()
}
