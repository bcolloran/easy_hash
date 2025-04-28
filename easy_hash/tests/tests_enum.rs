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

#[derive(EasyHash)]
struct InnerStruct {
    value: u32,
}

#[derive(EasyHash)]
enum InnerEnum {
    X,
    Y(u8),
    Z { val: u8 },
}

#[derive(EasyHash)]
enum TestEnumNested1 {
    A(InnerEnum),
    B { inner: InnerEnum },
    C(InnerStruct),
}

#[derive(EasyHash)]
enum TestEnumNested2 {
    A(InnerEnum),
    B { inner: InnerEnum },
    C(InnerStruct),
}

#[test]
fn test_nested_enum_equality() {
    let a1 = TestEnumNested1::A(InnerEnum::X);
    let a2 = TestEnumNested1::A(InnerEnum::X);
    assert_eq!(a1.ehash(), a2.ehash());

    let b1 = TestEnumNested1::B {
        inner: InnerEnum::Y(42),
    };
    let b2 = TestEnumNested1::B {
        inner: InnerEnum::Y(42),
    };
    assert_eq!(b1.ehash(), b2.ehash());
}

#[test]
fn test_nested_enum_inequality() {
    // Different inner enum variants
    let a1 = TestEnumNested1::A(InnerEnum::X);
    let a2 = TestEnumNested1::A(InnerEnum::Y(1));
    assert_ne!(a1.ehash(), a2.ehash());

    let b1 = TestEnumNested1::B {
        inner: InnerEnum::Z { val: 10 },
    };
    let b2 = TestEnumNested1::B {
        inner: InnerEnum::Z { val: 20 },
    };
    assert_ne!(b1.ehash(), b2.ehash());

    let c1 = TestEnumNested1::C(InnerStruct { value: 100 });
    let c2 = TestEnumNested2::C(InnerStruct { value: 100 });
    assert_ne!(c1.ehash(), c2.ehash());
}

#[test]
fn test_enum_with_struct_nested() {
    let a = TestEnumNested1::C(InnerStruct { value: 42 });
    let b = TestEnumNested1::C(InnerStruct { value: 42 });
    let c = TestEnumNested1::C(InnerStruct { value: 43 });

    assert_eq!(a.ehash(), b.ehash());
    assert_ne!(a.ehash(), c.ehash());
}

#[test]
fn test_empty_enum() {
    // Just compile test - we can't instantiate an empty enum
}

#[derive(EasyHash)]
enum EnumWithMultipleFields {
    A(u8, u16, u32),
    B { x: u8, y: u16, z: u32 },
}

#[test]
fn test_enum_with_multiple_fields() {
    let a1 = EnumWithMultipleFields::A(1, 2, 3);
    let a2 = EnumWithMultipleFields::A(1, 2, 3);
    let a3 = EnumWithMultipleFields::A(3, 2, 1);

    assert_eq!(a1.ehash(), a2.ehash());
    assert_ne!(a1.ehash(), a3.ehash());

    let b1 = EnumWithMultipleFields::B { x: 1, y: 2, z: 3 };
    let b2 = EnumWithMultipleFields::B { x: 1, y: 2, z: 3 };
    let b3 = EnumWithMultipleFields::B { x: 3, y: 2, z: 1 };

    assert_eq!(b1.ehash(), b2.ehash());
    assert_ne!(b1.ehash(), b3.ehash());
    assert_ne!(a1.ehash(), b1.ehash());
}
