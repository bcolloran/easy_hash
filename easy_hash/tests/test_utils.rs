use easy_hash::{split_u64, join_u32s, split_i64, u64_to_u32_slice};

#[test]
fn test_split_join_roundtrip() {
    let values = [0u64, 1, u32::MAX as u64, u64::MAX - 1, 0x1234_5678_9abc_def0];
    for &val in &values {
        let parts = split_u64(val);
        assert_eq!(join_u32s(parts[0], parts[1]), val);
    }
}

#[test]
fn test_u64_to_u32_slice() {
    let arr: [u64; 2] = [0x1122_3344_5566_7788, 0xaabb_ccdd_eeff_0011];
    let slice = u64_to_u32_slice(&arr);
    assert_eq!(slice.len(), 4);
    assert_eq!(slice[0], 0x5566_7788);
    assert_eq!(slice[1], 0x1122_3344);
    assert_eq!(slice[2], 0xeeff_0011);
    assert_eq!(slice[3], 0xaabb_ccdd);
}

#[test]
fn test_split_i64() {
    let values = [0i64, -1, i32::MAX as i64, i64::MIN + 1];
    for &val in &values {
        let parts = split_i64(val);
        let expected = split_u64(val as u64);
        assert_eq!(parts, expected);
    }
}
