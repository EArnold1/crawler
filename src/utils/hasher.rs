// This uses polynomial hash to get the key
fn convert_key(value: &str) -> u64 {
    let base: u64 = 31;
    let mut hash: u64 = 0;

    for c in value.chars() {
        let v = c as u64;
        hash = hash.wrapping_mul(base).wrapping_add(v);
    }

    hash
}

/// Simple division-based hash function to map a string(url) to a worker index.
pub fn division_hash(input: &str, hash_size: usize) -> usize {
    let key = convert_key(input);

    (key % (hash_size as u64)) as usize
}
