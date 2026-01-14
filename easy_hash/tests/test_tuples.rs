use easy_hash::EasyHash;
use test_case::test_case;

#[test_case(0.1, -0.1, 0, 1 ; "first case")]
fn test_tup_permute_floats_ne(a: f32, b: f32, c: u8, d: u8) {
    let aa = (a, b, c, d);
    let bb = (b, a, c, d);
    assert_ne!(aa.ehash(), bb.ehash());
}

#[test_case(0.0, -0.1, 0, 1 ; "first case")]
fn test_tup_permute_ints_ne(a: f32, b: f32, c: u8, d: u8) {
    let aa = (a, b, c, d);
    let bb = (a, b, d, c);
    assert_ne!(aa.ehash(), bb.ehash());
}
