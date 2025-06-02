pub fn fnv1a_hash(ptr: *const u8, len:usize) -> u32 {
    let mut hash: u32 = 2166136261;
    for i in 0..len {
        let byte = unsafe { *ptr.add(i) };
        hash ^= byte as u32;
        hash = hash.wrapping_mul(16777619);
    }
    hash
}

