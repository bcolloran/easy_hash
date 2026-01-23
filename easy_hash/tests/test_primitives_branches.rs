use easy_hash::fletcher::calc_fletcher64;
use easy_hash::{EasyHash, split_u64, type_salt};
use proptest::prelude::*;
use test_case::test_case;

#[test]
fn test_u8_max_branch() {
    let value = u8::MAX;
    let expected = calc_fletcher64(&[type_salt::<u8>(), (value as u32) | type_salt::<u8>()]);

    assert_eq!(value.ehash(), expected);
}

#[test]
fn test_u16_max_branch() {
    let value = u16::MAX;
    let expected = calc_fletcher64(&[type_salt::<u16>(), (value as u32) | type_salt::<u16>()]);

    assert_eq!(value.ehash(), expected);
}

#[test]
fn test_u32_max_branch() {
    let value = u32::MAX;
    let expected = calc_fletcher64(&[type_salt::<u32>(), value, type_salt::<u32>()]);

    assert_eq!(value.ehash(), expected);
}

#[test]
fn test_u64_max_branch() {
    let value = u64::MAX;
    let mut checksum = easy_hash::fletcher::Fletcher64::new();
    checksum.update(&[type_salt::<u64>()]);
    checksum.update(&split_u64(value));
    checksum.update(&[type_salt::<u64>()]);

    assert_eq!(value.ehash(), checksum.value());
}

#[test]
fn test_i8_negative_one_branch() {
    let value = -1i8;
    let expected = calc_fletcher64(&[type_salt::<i8>(), (value as u32) | type_salt::<i8>()]);

    assert_eq!(value.ehash(), expected);
}

#[test_case(0i8; "zero")]
#[test_case(42i8; "positive")]
fn test_i8_non_max_values(value: i8) {
    let expected = calc_fletcher64(&[type_salt::<i8>(), value as u32]);

    assert_eq!(value.ehash(), expected);
}

#[test]
fn test_i16_negative_one_branch() {
    let value = -1i16;
    let expected = calc_fletcher64(&[type_salt::<i16>(), (value as u32) | type_salt::<i16>()]);

    assert_eq!(value.ehash(), expected);
}

#[test_case(0i16; "zero")]
#[test_case(1234i16; "positive")]
fn test_i16_non_max_values(value: i16) {
    let expected = calc_fletcher64(&[type_salt::<i16>(), value as u32]);

    assert_eq!(value.ehash(), expected);
}

#[test]
fn test_i32_negative_one_branch() {
    let value = -1i32;
    let expected = calc_fletcher64(&[type_salt::<i32>(), value as u32, type_salt::<i32>()]);

    assert_eq!(value.ehash(), expected);
}

#[test]
fn test_i64_negative_one_branch() {
    let value = -1i64;
    let mut checksum = easy_hash::fletcher::Fletcher64::new();
    checksum.update(&[type_salt::<i64>()]);
    checksum.update(&split_u64(value as u64));
    checksum.update(&[type_salt::<i64>()]);

    assert_eq!(value.ehash(), checksum.value());
}

#[cfg(target_pointer_width = "64")]
#[test]
fn test_usize_max_branch() {
    let value = usize::MAX;
    let mut checksum = easy_hash::fletcher::Fletcher64::new();
    checksum.update(&[type_salt::<usize>()]);
    checksum.update(&split_u64(value as u64));
    checksum.update(&[type_salt::<usize>()]);

    assert_eq!(value.ehash(), checksum.value());
}

#[cfg(target_pointer_width = "64")]
#[test]
fn test_isize_negative_one_branch() {
    let value = -1isize;
    let mut checksum = easy_hash::fletcher::Fletcher64::new();
    checksum.update(&[type_salt::<isize>()]);
    checksum.update(&split_u64(value as u64));
    checksum.update(&[type_salt::<isize>()]);

    assert_eq!(value.ehash(), checksum.value());
}

#[cfg(target_pointer_width = "64")]
#[test_case(0isize; "zero")]
#[test_case(42isize; "positive")]
fn test_isize_non_max_values(value: isize) {
    let mut checksum = easy_hash::fletcher::Fletcher64::new();
    checksum.update(&[type_salt::<isize>()]);
    checksum.update(&split_u64(value as u64));

    assert_eq!(value.ehash(), checksum.value());
}

proptest! {
    #[test]
    fn prop_u64_hash_matches_manual(value in prop_oneof![Just(u64::MAX), any::<u64>()]) {
        let mut checksum = easy_hash::fletcher::Fletcher64::new();
        checksum.update(&[type_salt::<u64>()]);
        checksum.update(&split_u64(value));
        if value == u64::MAX {
            checksum.update(&[type_salt::<u64>()]);
        }

        prop_assert_eq!(value.ehash(), checksum.value());
    }

    #[test]
    fn prop_i64_hash_matches_manual(value in prop_oneof![Just(-1i64), any::<i64>()]) {
        let mut checksum = easy_hash::fletcher::Fletcher64::new();
        checksum.update(&[type_salt::<i64>()]);
        checksum.update(&split_u64(value as u64));
        if value as u64 == u64::MAX {
            checksum.update(&[type_salt::<i64>()]);
        }

        prop_assert_eq!(value.ehash(), checksum.value());
    }
}
