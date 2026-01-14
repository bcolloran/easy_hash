use const_fnv1a_hash::{fnv1a_hash_32, fnv1a_hash_str_64};

pub const fn type_salt_generic<T, G>() -> u32 {
    let main_hash = fnv1a_hash_str_64(std::any::type_name::<T>());
    let generic_hash = fnv1a_hash_str_64(std::any::type_name::<G>());

    // Combine the two u64 hashes into a single u32 by converting to bytes and hashing
    let main_bytes = main_hash.to_ne_bytes();
    let generic_bytes = generic_hash.to_ne_bytes();

    // YUCK! written by llm, but works in const fn context
    let combined = [
        main_bytes[0],
        main_bytes[1],
        main_bytes[2],
        main_bytes[3],
        main_bytes[4],
        main_bytes[5],
        main_bytes[6],
        main_bytes[7],
        generic_bytes[0],
        generic_bytes[1],
        generic_bytes[2],
        generic_bytes[3],
        generic_bytes[4],
        generic_bytes[5],
        generic_bytes[6],
        generic_bytes[7],
    ];

    fnv1a_hash_32(&combined, None)
}
