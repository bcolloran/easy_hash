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

#[derive(EasyHash)]
struct TestTupStruct(f32, f32, u8, u8);

#[test_case(0.0, -0.1, 0, 1 ; "first case")]
fn test_tupstruct_ne_tup_when_reordered(a: f32, b: f32, c: u8, d: u8) {
    let aa = TestTupStruct(a, b, c, d);
    let bb = (a, b, c, d);
    assert_ne!((&bb, &aa).ehash(), (&aa, &bb).ehash());
}

#[derive(EasyHash)]
struct TestTupOptionStruct(Option<f32>);

#[derive(EasyHash)]
enum TestEnum1 {
    A,
    B,
    C,
}

#[derive(EasyHash)]
enum TestEnum2 {
    A,
    B,
    C,
}

#[test]
fn test_enum_self_eq() {
    let a_a = TestEnum1::A;
    let a_b = TestEnum1::B;
    let a_c = TestEnum1::C;
    assert_eq!(a_a.ehash(), a_a.ehash());
    let b_a = TestEnum2::A;
    let b_b = TestEnum2::B;
    let b_c = TestEnum2::C;

    // variants with the same name in different enums should not be equal
    assert_ne!(a_a.ehash(), b_a.ehash());
    assert_ne!(a_b.ehash(), b_b.ehash());
    assert_ne!(a_c.ehash(), b_c.ehash());

    // variants with different names in different enums should not be equal
    assert_ne!(a_a.ehash(), a_b.ehash());
    assert_ne!(a_a.ehash(), b_a.ehash());
}

#[derive(EasyHash)]
enum TestEnum3 {
    A(u8),
    B(u8),
}

#[derive(EasyHash)]
enum TestEnum4 {
    A(u8),
    B((u8, u8)),
    // C { x: u8, y: u8 },
}

#[test]
fn test_enum_with_data() {
    let a_3_0 = TestEnum3::A(0);
    let a_3_1 = TestEnum3::A(1);
    let b_3_0 = TestEnum3::B(0);
    let b_3_1 = TestEnum3::B(1);

    // variants with the same name and enums but different data
    // must not be equal
    assert_ne!(a_3_0.ehash(), a_3_1.ehash());
    assert_ne!(b_3_0.ehash(), b_3_1.ehash());

    // variants with different names but same data must not be equal
    assert_ne!(a_3_0.ehash(), b_3_0.ehash());

    // variants with the same name and enums but different data
    // must not be equal
    let a_4_0 = TestEnum4::A(0);
    let b_4_00 = TestEnum4::B((0, 0));
    assert_ne!(a_4_0.ehash(), b_4_00.ehash());

    // variants with different names in different enums should not be equal
    // assert_ne!(a_a.ehash(), a_b.ehash());
    // assert_ne!(a_a.ehash(), b_a.ehash());
}

#[derive(EasyHash)]
enum TestEnum5 {
    A { x: u8, y: u8 },
    B { x: u8, y: u8 },
    C { x: u8, y: u8 },
}

#[derive(EasyHash)]
enum TestEnum6 {
    A { x: u8, y: u8 },
    B { x: u8, y: u8 },
    C { x: u8, y: u8 },
}

#[test]
fn test_enum_with_struct_data() {
    let a_5_0 = TestEnum5::A { x: 0, y: 0 };
    let a_6_0 = TestEnum6::A { x: 0, y: 0 };

    let b_5_0 = TestEnum5::B { x: 0, y: 0 };
    let b_5_1 = TestEnum5::B { x: 1, y: 0 };
    let b_5_1_permute = TestEnum5::B { y: 0, x: 1 };

    // variants with the same name and variants but different data
    // must not be equal
    assert_ne!(a_5_0.ehash(), a_6_0.ehash());

    // should be equal if fields are permuted
    assert_eq!(b_5_1.ehash(), b_5_1_permute.ehash());

    // variants with different names but same data must not be equal
    assert_ne!(a_5_0.ehash(), b_5_0.ehash());
}

#[derive(EasyHash)]
struct TestStructIgnore {
    a: u8,
    b: f32,
    #[easy_hash_ignore]
    c: f32,
    d: u8,
}

#[test]
fn test_struct_with_ignored_field() {
    let a = TestStructIgnore {
        a: 0,
        b: 1.2,
        c: 3.4,
        d: 4,
    };
    let b = TestStructIgnore {
        a: 0,
        b: 1.2,
        c: 9803.4,
        d: 4,
    };
    // should be equal if only ignored fields are different
    assert_eq!(a.ehash(), b.ehash());
}
