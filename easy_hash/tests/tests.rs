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

fn test_tup_of_tupstruct_and_tup_ne_tup_when_reordered() {
    let a = TestTupStruct(0.0, -0.1, 0, 1);
    let xxx = (0.0, -0.1, 0, 1);

    assert_ne!((&xxx, &a).ehash(), (&a, &xxx).ehash());
}

#[derive(EasyHash)]
struct TestTupOptionStruct(Option<f32>);

#[derive(EasyHash)]
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
