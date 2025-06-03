use easy_hash::{join_u32s, split_i64, split_u64, u64_to_u32_slice};
use test_case::test_case;

#[test_case(0 ; "0u64")]
#[test_case(1 ; "1u64")]
#[test_case(u32::MAX as u64 ; "u32::MAX as u64")]
#[test_case(u64::MAX ; "u64::MAX")]
#[test_case(u64::MAX - 1 ; "u64::MAX - 1")]
#[test_case(0x1234_5678_9abc_def0 ; "0x1234_5678_9abc_def0")]
fn test_split_u64_roundtrip(val: u64) {
    let parts = split_u64(val);
    assert_eq!(join_u32s(parts[0], parts[1]), val);
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

#[test_case(0i64 ; "0i64")]
#[test_case(1 ; "1i64")]
#[test_case(-1 ; "negative 1 i64")]
#[test_case(u32::MAX as i64 ; "u32::MAX as i64")]
#[test_case(i64::MAX ; "i64::MAX")]
#[test_case(i64::MIN ; "i64::MIN")]
#[test_case(i64::MAX - 1 ; "i64::MAX - 1")]
#[test_case(i64::MIN + 1 ; "i64::MIN + 1")]
#[test_case(0x1234_5678_9abc_def0 ; "0x1234_5678_9abc_def0")]
fn test_split_i64_roundtrip(val: i64) {
    let parts = split_i64(val);
    assert_eq!(join_u32s(parts[0], parts[1]) as i64, val);
}
