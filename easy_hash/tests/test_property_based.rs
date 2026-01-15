use bytemuck::cast_slice;
use easy_hash::{EasyHash, join_u32s, split_i64, split_u64, type_salt, u64_to_u32_slice};
use fletcher::Fletcher64;
use proptest::prelude::*;

fn manual_option_some_hash(value: u64) -> u64 {
    let parts = split_u64(value.ehash());
    let mut checksum = Fletcher64::new();
    checksum.update(&[type_salt::<u64>(), parts[0], parts[1]]);
    checksum.value()
}

fn manual_vec_hash(values: &[u64]) -> u64 {
    let mut checksum = Fletcher64::new();
    checksum.update(&[type_salt::<Vec<u64>>()]);
    let hashes: Vec<u64> = values.iter().map(|value| value.ehash()).collect();
    checksum.update(u64_to_u32_slice(&hashes));
    checksum.value()
}

fn manual_tuple_hash(left: u64, right: u32) -> u64 {
    let mut checksum = Fletcher64::new();
    checksum.update(&[type_salt::<(u64, u32)>()]);
    let hashes = [left.ehash(), right.ehash()];
    checksum.update(u64_to_u32_slice(&hashes));
    checksum.value()
}

fn manual_u64_slice_hash(values: &[u64]) -> u64 {
    let mut checksum = Fletcher64::new();
    checksum.update(&[type_salt::<&[u64]>()]);
    checksum.update(u64_to_u32_slice(values));
    checksum.value()
}

fn manual_u64_array_hash(values: &[u64; 4]) -> u64 {
    let mut checksum = Fletcher64::new();
    checksum.update(&[type_salt::<[u64; 4]>()]);
    checksum.update(u64_to_u32_slice(values));
    checksum.value()
}

proptest! {
    #[test]
    fn prop_split_u64_roundtrip(value in any::<u64>()) {
        let parts = split_u64(value);
        prop_assert_eq!(join_u32s(parts[0], parts[1]), value);
    }

    #[test]
    fn prop_split_i64_roundtrip(value in any::<i64>()) {
        let parts = split_i64(value);
        prop_assert_eq!(join_u32s(parts[0], parts[1]) as i64, value);
    }

    #[test]
    fn prop_u64_to_u32_slice_roundtrip(values in prop::collection::vec(any::<u64>(), 0..128)) {
        let parts = u64_to_u32_slice(&values);
        prop_assert_eq!(parts.len(), values.len() * 2);

        let rebuilt = cast_slice::<u32, u64>(parts);
        prop_assert_eq!(rebuilt, values.as_slice());
    }

    #[test]
    fn prop_reference_hash_matches_value(value in any::<u64>()) {
        prop_assert_eq!(value.ehash(), (&value).ehash());
    }

    #[test]
    fn prop_option_some_hash_matches_manual(value in any::<u64>()) {
        prop_assert_eq!(Some(value).ehash(), manual_option_some_hash(value));
    }

    #[test]
    fn prop_vec_hash_matches_manual(values in prop::collection::vec(any::<u64>(), 0..64)) {
        prop_assert_eq!(values.ehash(), manual_vec_hash(&values));
    }

    #[test]
    fn prop_tuple_hash_matches_manual(left in any::<u64>(), right in any::<u32>()) {
        prop_assert_eq!((left, right).ehash(), manual_tuple_hash(left, right));
    }

    #[test]
    fn prop_u64_slice_hash_matches_manual(values in prop::collection::vec(any::<u64>(), 0..64)) {
        prop_assert_eq!(values.as_slice().ehash(), manual_u64_slice_hash(&values));
    }

    #[test]
    fn prop_u64_array_hash_matches_manual(values in prop::array::uniform4(any::<u64>())) {
        prop_assert_eq!(values.ehash(), manual_u64_array_hash(&values));
    }
}
