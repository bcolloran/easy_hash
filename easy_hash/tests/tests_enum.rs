use easy_hash::EasyHash;

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
    C(u8, u8),
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

    // variants with the same name and same data but from different enums
    // should NOT be equal
    let a_3_0 = TestEnum3::A(0);
    let a_4_0 = TestEnum4::A(0);
    assert_ne!(a_3_0.ehash(), a_4_0.ehash());
}

#[test]
fn test_enum_with_tup_data_permutation() {
    // variants with permuted data
    // should NOT be equal
    let a = TestEnum4::C(0, 1);
    let b = TestEnum4::C(1, 0);
    assert_ne!(a.ehash(), b.ehash());
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
fn test_enum_with_struct_data_equality() {
    let a = TestEnum5::A { x: 123, y: 0 };
    let b = TestEnum5::A { x: 123, y: 0 };

    assert_eq!(a.ehash(), b.ehash());
}

#[test]
fn test_enum_with_struct_data() {
    let a_5_0 = TestEnum5::A { x: 0, y: 0 };
    let a_6_0 = TestEnum6::A { x: 0, y: 0 };

    let b_5_0 = TestEnum5::B { x: 0, y: 0 };

    // variants with the same name but from different enums
    // should NOT be equal
    assert_ne!(a_5_0.ehash(), a_6_0.ehash());

    // variants with different names but same data must not be equal
    assert_ne!(a_5_0.ehash(), b_5_0.ehash());
}

#[test]
fn test_enum_with_struct_data_order_permutation() {
    let a = TestEnum5::B { x: 1, y: 0 };
    let flip = TestEnum5::B { x: 0, y: 1 };

    // should NOT be equal if data in fields are permuted
    assert_ne!(a.ehash(), flip.ehash());
}
