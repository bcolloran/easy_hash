use easy_hash::EasyHash;

#[derive(EasyHash)]
struct TestStruct<T: EasyHash> {
    a: T,
}
impl<T: EasyHash> TestStruct<T> {
    fn new(a: T) -> Self {
        Self { a }
    }
}

#[test]
fn test_generic_struct_same_type() {
    let s1 = TestStruct::new(10u8);
    let s2 = TestStruct::new(10u8);
    let s3 = TestStruct::new(20u8);
    assert_eq!(s1.ehash(), s2.ehash());
    assert_ne!(s1.ehash(), s3.ehash());
}

#[test]
fn test_generic_struct_different_types_different_sizes() {
    let s1 = TestStruct::new(10u8);
    let s2 = TestStruct::new(10u16);
    assert_ne!(s1.ehash(), s2.ehash());
}

#[test]
fn test_generic_struct_different_types_same_bits() {
    let s1 = TestStruct::new(0u32);
    let s2 = TestStruct::new(0i32);
    let s3 = TestStruct::new(0f32);
    let s4 = TestStruct::new([0u8; 4]);
    assert_ne!(s1.ehash(), s2.ehash());
    assert_ne!(s1.ehash(), s3.ehash());
    assert_ne!(s1.ehash(), s4.ehash());
    assert_ne!(s2.ehash(), s3.ehash());
    assert_ne!(s2.ehash(), s4.ehash());
    assert_ne!(s3.ehash(), s4.ehash());
}

#[derive(EasyHash)]
struct ChildType(u32);

#[test]
fn test_generic_struct_with_custom_type() {
    let s1 = TestStruct::new(ChildType(10));
    let s2 = TestStruct::new(ChildType(10));
    let s3 = TestStruct::new(ChildType(20));
    assert_eq!(s1.ehash(), s2.ehash());
    assert_ne!(s1.ehash(), s3.ehash());
}

#[test]
fn test_generic_struct_with_custom_type_same_bits() {
    let s1 = TestStruct::new(ChildType(10));
    let s2 = TestStruct::new(10u32);
    assert_ne!(s1.ehash(), s2.ehash());
}

#[test]
fn test_generic_struct_with_reference_type() {
    let value = 10u32;
    let s1 = TestStruct::new(&value);
    let s2 = TestStruct::new(&value);
    let different_value = 20u32;
    let s3 = TestStruct::new(&different_value);
    assert_eq!(s1.ehash(), s2.ehash());
    assert_ne!(s1.ehash(), s3.ehash());
}
