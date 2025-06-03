use easy_hash::EasyHash;
use test_case::test_case;

#[test_case(0 ; "for 0")]
#[test_case(1  ; "for 1")]
#[test_case(107  ; "for 107")]
fn test_salted_uints(x: u8) {
    assert_ne!((x as u8).ehash(), x as u64);
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

#[test_case("hello" ; "for hello")]
#[test_case("" ; "for empty string")]
#[test_case("a" ; "for single char")]
#[test_case("hello world" ; "for string with space")]
#[test_case("!@#$%^&*()" ; "for special chars")]
#[test_case("你好" ; "for unicode")]
#[test_case("a\nb\tc" ; "for escape sequences")]
#[test_case("\0" ; "for null byte")]
#[test_case("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" ; "for long string")]
fn test_str_special_cases(s: &str) {
    // Test that the hash is consistent for the same value
    assert_eq!(s.ehash(), s.ehash());

    // Test that different strings have different hashes
    for other in ["", "a", "hello", "world", "你好", "\0"] {
        if s != other {
            assert_ne!(s.ehash(), other.ehash());
        }
    }

    // Test that the hash is different from a string with the same first char
    if !s.is_empty() {
        let first_char = s.chars().next().unwrap().to_string();
        if s != first_char.as_str() {
            assert_ne!(s.ehash(), first_char.ehash());
        }
    }

    // Test that the hash is different from a prefix of the string
    if s.len() > 1 {
        let prefix = &s[0..s.len() / 2];
        assert_ne!(s.ehash(), prefix.ehash());
    }
}

#[test]
fn test_str_case_sensitivity() {
    let lowercase = "hello";
    let uppercase = "HELLO";
    assert_ne!(lowercase.ehash(), uppercase.ehash());
}

#[test]
fn test_str_vs_string() {
    let s_str = "test string";
    let s_string = String::from("test string");
    assert_ne!(s_str.ehash(), s_string.ehash());
}

#[test]
fn test_str_slices() {
    let s = "hello world";
    let slice1 = &s[0..5];
    let slice2 = &s[6..];

    assert_ne!(s.ehash(), slice1.ehash());
    assert_ne!(s.ehash(), slice2.ehash());
    assert_ne!(slice1.ehash(), slice2.ehash());
}

#[test]
fn test_identical_looking_but_different_unicode() {
    // Cyrillic 'а' (U+0430) vs Latin 'a' (U+0061)
    let s1 = "а";
    let s2 = "a";
    assert_ne!(s1.ehash(), s2.ehash());

    // Regular space vs non-breaking space
    let s3 = " ";
    let s4 = "\u{00A0}";
    assert_ne!(s3.ehash(), s4.ehash());
}

// #[test]
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

#[derive(EasyHash)]
struct TestUnitStructA;

#[derive(EasyHash)]
struct TestUnitStructB;

#[test]
fn test_unit_structs() {
    let a_1 = TestUnitStructA;
    let a_2 = TestUnitStructA;
    let b_1 = TestUnitStructB;

    assert_eq!(a_1.ehash(), a_2.ehash());
    assert_ne!(a_1.ehash(), b_1.ehash());
}

#[derive(EasyHash)]
struct TestStruct {
    a: f32,
    b: f32,
    c: u8,
    d: u8,
}

// #[test]
#[test_case(0.0, -0.1, 0, 1 ; "first case")]
fn test_structs_permute_floats_fields(a: f32, bbb: f32, c: u8, d: u8) {
    let aa = TestStruct {
        a: a,
        b: bbb,
        c: c,
        d: d,
    };
    let bb = TestStruct {
        a: bbb,
        b: a,
        c: c,
        d: d,
    };
    assert_ne!(aa.ehash(), bb.ehash());
}

#[test_case(0.0, -0.1, 0, 1 ; "first case")]
fn test_structs_permute_int_fields(a: f32, b: f32, ccc: u8, d: u8) {
    let aa = TestStruct {
        a: a,
        b: b,
        c: ccc,
        d: d,
    };
    let bb = TestStruct {
        a: a,
        b: b,
        c: d,
        d: ccc,
    };
    assert_ne!(aa.ehash(), bb.ehash());
}

#[test_case(0.0, -0.1, 0, 1 ; "first case")]
fn test_structs_not_equal_to_tup_with_same_data(a: f32, b: f32, c: u8, d: u8) {
    let aa = TestStruct {
        a: a,
        b: b,
        c: c,
        d: d,
    };
    let bb = (a, b, c, d);
    assert_ne!(aa.ehash(), bb.ehash());
}

#[derive(EasyHash)]
struct TestTupStruct(f32, f32, u8, u8);

#[test]
fn test_tup_structs_permute_floats_fields() {
    let a = TestTupStruct(-1.0, 1.0, 0, 0);
    let b = TestTupStruct(1.0, -1.0, 0, 0);
    assert_ne!(a.ehash(), b.ehash());
}
#[test]
fn test_tup_structs_permute_int_fields() {
    let a = TestTupStruct(0.0, -0.1, 0, 1);
    let b = TestTupStruct(0.0, -0.1, 1, 0);
    assert_ne!(a.ehash(), b.ehash());
}

#[derive(EasyHash)]
struct TestStructTwo {
    a: f32,
    b: f32,
    c: u8,
    d: u8,
}

#[test_case(0.0, -0.1, 0, 1 ; "first case")]
fn test_different_types_with_same_data_not_equal(a: f32, b: f32, c: u8, d: u8) {
    let aa = TestStruct {
        a: a,
        b: b,
        c: c,
        d: d,
    };
    let bb = TestStructTwo {
        a: a,
        b: b,
        c: c,
        d: d,
    };
    assert_ne!(aa.ehash(), bb.ehash());
}

#[test_case(0.0, -0.1, 0, 1 ; "first case")]
fn test_tup_of_struct(a: f32, b: f32, c: u8, d: u8) {
    let aa = TestStruct {
        a: a,
        b: b,
        c: c,
        d: d,
    };
    let bb = TestStructTwo {
        a: a,
        b: b,
        c: c,
        d: d,
    };
    assert_eq!((&aa, &bb).ehash(), (&aa, &bb).ehash());
}

#[test_case(0.0, -0.1, 0, 1 ; "first case")]
fn test_tup_of_struct_ne_when_reordered(a: f32, b: f32, c: u8, d: u8) {
    let aa = TestStruct {
        a: a,
        b: b,
        c: c,
        d: d,
    };
    let bb = TestStructTwo {
        a: a,
        b: b,
        c: c,
        d: d,
    };
    assert_ne!((&bb, &aa).ehash(), (&aa, &bb).ehash());
}

#[test_case(0.0, -0.1, 0, 1 ; "first case")]
fn test_vec_of_struct_ne_when_reordered(a: f32, b: f32, c: u8, d: u8) {
    let aa = TestStruct {
        a: a,
        b: b,
        c: c,
        d: d,
    };
    let bb = TestStruct {
        a: a,
        b: b,
        c: c,
        d: d + 1,
    };
    let v1 = vec![&bb, &aa];
    let v2 = vec![&aa, &bb];
    assert_ne!(v1.ehash(), v2.ehash());
}

#[test]
fn test_tup_of_tupstruct_and_tup_ne_tup_when_reordered() {
    let a = TestTupStruct(0.0, -0.1, 0, 1);
    let xxx = (0.0, -0.1, 0, 1);

    assert_ne!((&xxx, &a).ehash(), (&a, &xxx).ehash());
}

#[derive(EasyHash)]
struct TestTupOptionStruct(Option<f32>);

#[derive(EasyHash)]
#[allow(dead_code)]
struct TestStructIgnore {
    a: u8,
    b: f32,
    #[easy_hash_ignore]
    c: f32,
    x: u8,
}

#[test]
fn test_struct_with_ignored_field() {
    let a = TestStructIgnore {
        a: 0,
        b: 1.2,
        c: 3.4,
        x: 4,
    };
    let b = TestStructIgnore {
        a: 0,
        b: 1.2,
        c: 9803.4,
        x: 4,
    };
    // should be equal if only ignored fields are different
    assert_eq!(a.ehash(), b.ehash());
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
