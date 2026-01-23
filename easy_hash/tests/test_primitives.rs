use easy_hash::EasyHash;
use test_case::test_case;

#[test_case(0 ; "for 0")]
#[test_case(1  ; "for 1")]
#[test_case(107  ; "for 107")]
fn test_salted_uints(x: u8) {
    assert_ne!(x.ehash(), x as u64);
}

#[test_case((0,1) ; "for 0 1")]
#[test_case((0,u32::MAX)  ; "for 0 max")]
#[test_case((1,u32::MAX)  ; "for 1 max")]
#[test_case((1,u32::MIN)  ; "for 1 min")]
fn test_u32_problem_cases(ab: (u32, u32)) {
    assert_ne!((ab.0).ehash(), (ab.1).ehash());
}

#[test_case((0,1) ; "for 0 1")]
#[test_case((1,-1)  ; "for 1 neg1")]
#[test_case((0,-1)  ; "for 0 neg1")]
#[test_case((0,i32::MAX)  ; "for 0 max")]
#[test_case((0,i32::MIN)  ; "for 0 min")]
#[test_case((1,i32::MAX)  ; "for 1 max")]
#[test_case((1,i32::MIN)  ; "for 1 min")]
#[test_case((-1,i32::MAX)  ; "for neg1 max")]
#[test_case((-1,i32::MIN)  ; "for neg1 min")]
fn test_i32_problem_cases(ab: (i32, i32)) {
    assert_ne!((ab.0).ehash(), (ab.1).ehash());
}

#[test_case(0.0 ; "for zero")]
#[test_case(-0.0 ; "for negative zero")]
#[test_case(f32::INFINITY ; "for infinity")]
#[test_case(f32::NEG_INFINITY ; "for negative infinity")]
#[test_case(f32::NAN ; "for NaN")]
#[test_case(f32::MIN ; "for min value")]
#[test_case(f32::MAX ; "for max value")]
#[test_case(f32::MIN_POSITIVE ; "for min positive value")]
#[test_case(1.0 ; "for one")]
#[test_case(-1.0 ; "for negative one")]
fn test_f32_special_values(x: f32) {
    // Test that the hash is consistent for the same value
    assert_eq!(x.ehash(), x.ehash());

    // Test that the hash is different from the value itself
    if !x.is_nan() {
        // NaN doesn't equal itself
        assert_ne!(x.ehash(), x as u64);
    }

    for special_val in [
        0.0,
        -0.0,
        1.0,
        -1.0,
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::NAN,
        f32::MIN,
        f32::MAX,
        f32::MIN_POSITIVE,
    ] {
        // Test that the hash is different from special values
        if x != special_val && !x.is_nan() {
            assert_ne!(x.ehash(), special_val.ehash());
        }
    }

    // Test that the hash of x is different from -x (except for 0.0 and -0.0)
    if x != 0.0 && x != -0.0 && !x.is_nan() {
        assert_ne!(x.ehash(), (-x).ehash());
    }
}

#[test_case(0i32, 0u64 ; "i32 vs u64")]
#[test_case(0i32, 0usize ; "i32 vs usize")]
#[test_case(0i32, false ; "i32 vs bool")]
#[test_case(0i32, () ; "i32 vs unit")]
#[test_case(0u64, 0usize ; "u64 vs usize")]
#[test_case(0u64, false ; "u64 vs bool")]
#[test_case(0u64, () ; "u64 vs unit")]
#[test_case(0usize, false ; "usize vs bool")]
#[test_case(0usize, () ; "usize vs unit")]
#[test_case(false, () ; "bool vs unit")]
fn test_zero_values_different_types<A: EasyHash, B: EasyHash>(a: A, b: B) {
    assert_ne!(a.ehash(), b.ehash());
}
