use easy_hash::EasyHash;
use std::marker::PhantomData;

// Test basic PhantomData hashing
#[test]
fn test_phantom_data_hash() {
    let p1: PhantomData<u32> = PhantomData;
    let p2: PhantomData<u32> = PhantomData;
    assert_eq!(p1.ehash(), p2.ehash());
}

// Test that different PhantomData types have different hashes
#[test]
fn test_phantom_data_different_types() {
    let p1: PhantomData<u32> = PhantomData;
    let p2: PhantomData<u64> = PhantomData;
    assert_ne!(p1.ehash(), p2.ehash());
}

// Test struct with PhantomData field
#[derive(EasyHash)]
struct WithPhantomData<T> {
    value: u32,
    _phantom: PhantomData<T>,
}

impl<T> WithPhantomData<T> {
    fn new(value: u32) -> Self {
        Self {
            value,
            _phantom: PhantomData,
        }
    }
}

#[test]
fn test_struct_with_phantom_data_same_type() {
    let s1 = WithPhantomData::<u32>::new(10);
    let s2 = WithPhantomData::<u32>::new(10);
    assert_eq!(s1.ehash(), s2.ehash());
}

#[test]
fn test_struct_with_phantom_data_different_values() {
    let s1 = WithPhantomData::<u32>::new(10);
    let s2 = WithPhantomData::<u32>::new(20);
    assert_ne!(s1.ehash(), s2.ehash());
}

#[test]
fn test_struct_with_phantom_data_different_phantom_types() {
    let s1 = WithPhantomData::<u32>::new(10);
    let s2 = WithPhantomData::<u64>::new(10);
    // Different phantom types should result in different hashes
    assert_ne!(s1.ehash(), s2.ehash());
}

// Test struct with only PhantomData
#[derive(EasyHash)]
struct OnlyPhantomData<T> {
    _phantom: PhantomData<T>,
}

#[test]
fn test_struct_only_phantom_data_same_type() {
    let s1 = OnlyPhantomData::<u32> {
        _phantom: PhantomData,
    };
    let s2 = OnlyPhantomData::<u32> {
        _phantom: PhantomData,
    };
    assert_eq!(s1.ehash(), s2.ehash());
}

#[test]
fn test_struct_only_phantom_data_different_types() {
    let s1 = OnlyPhantomData::<u32> {
        _phantom: PhantomData,
    };
    let s2 = OnlyPhantomData::<u64> {
        _phantom: PhantomData,
    };
    assert_ne!(s1.ehash(), s2.ehash());
}

// Test struct with multiple PhantomData fields
#[derive(EasyHash)]
struct MultiplePhantomData<T, U> {
    value: i32,
    _phantom1: PhantomData<T>,
    _phantom2: PhantomData<U>,
}

#[test]
fn test_multiple_phantom_data_same_types() {
    let s1 = MultiplePhantomData::<u32, u64> {
        value: 42,
        _phantom1: PhantomData,
        _phantom2: PhantomData,
    };
    let s2 = MultiplePhantomData::<u32, u64> {
        value: 42,
        _phantom1: PhantomData,
        _phantom2: PhantomData,
    };
    assert_eq!(s1.ehash(), s2.ehash());
}

#[test]
fn test_multiple_phantom_data_different_first_type() {
    let s1 = MultiplePhantomData::<u32, u64> {
        value: 42,
        _phantom1: PhantomData,
        _phantom2: PhantomData,
    };
    let s2 = MultiplePhantomData::<i32, u64> {
        value: 42,
        _phantom1: PhantomData,
        _phantom2: PhantomData,
    };
    assert_ne!(s1.ehash(), s2.ehash());
}

#[test]
fn test_multiple_phantom_data_different_second_type() {
    let s1 = MultiplePhantomData::<u32, u64> {
        value: 42,
        _phantom1: PhantomData,
        _phantom2: PhantomData,
    };
    let s2 = MultiplePhantomData::<u32, i64> {
        value: 42,
        _phantom1: PhantomData,
        _phantom2: PhantomData,
    };
    assert_ne!(s1.ehash(), s2.ehash());
}

// Test tuple struct with PhantomData
#[derive(EasyHash)]
struct TupleWithPhantomData<T>(u32, PhantomData<T>);

#[test]
fn test_tuple_struct_with_phantom_data() {
    let s1 = TupleWithPhantomData::<u32>(10, PhantomData);
    let s2 = TupleWithPhantomData::<u32>(10, PhantomData);
    assert_eq!(s1.ehash(), s2.ehash());
}

#[test]
fn test_tuple_struct_with_phantom_data_different_types() {
    let s1 = TupleWithPhantomData::<u32>(10, PhantomData);
    let s2 = TupleWithPhantomData::<u64>(10, PhantomData);
    assert_ne!(s1.ehash(), s2.ehash());
}

// Test enum with PhantomData
#[derive(EasyHash)]
enum EnumWithPhantomData<T> {
    Variant1(u32, PhantomData<T>),
    Variant2 {
        value: u32,
        _phantom: PhantomData<T>,
    },
}

#[test]
fn test_enum_with_phantom_data_same_variant() {
    let e1 = EnumWithPhantomData::<u32>::Variant1(10, PhantomData);
    let e2 = EnumWithPhantomData::<u32>::Variant1(10, PhantomData);
    assert_eq!(e1.ehash(), e2.ehash());
}

#[test]
fn test_enum_with_phantom_data_different_types() {
    let e1 = EnumWithPhantomData::<u32>::Variant1(10, PhantomData);
    let e2 = EnumWithPhantomData::<u64>::Variant1(10, PhantomData);
    assert_ne!(e1.ehash(), e2.ehash());
}

#[test]
fn test_enum_with_phantom_data_named_variant() {
    let e1 = EnumWithPhantomData::<u32>::Variant2 {
        value: 20,
        _phantom: PhantomData,
    };
    let e2 = EnumWithPhantomData::<u32>::Variant2 {
        value: 20,
        _phantom: PhantomData,
    };
    assert_eq!(e1.ehash(), e2.ehash());
}

// Test nested PhantomData
#[derive(EasyHash)]
struct NestedPhantomData<T> {
    _phantom: PhantomData<PhantomData<T>>,
}

#[test]
fn test_nested_phantom_data() {
    let s1 = NestedPhantomData::<u32> {
        _phantom: PhantomData,
    };
    let s2 = NestedPhantomData::<u32> {
        _phantom: PhantomData,
    };
    assert_eq!(s1.ehash(), s2.ehash());
}

#[test]
fn test_nested_phantom_data_different_types() {
    let s1 = NestedPhantomData::<u32> {
        _phantom: PhantomData,
    };
    let s2 = NestedPhantomData::<u64> {
        _phantom: PhantomData,
    };
    assert_ne!(s1.ehash(), s2.ehash());
}

// Test PhantomData with complex types
#[derive(EasyHash)]
struct ComplexType {
    x: u32,
    y: u32,
}

#[derive(EasyHash)]
struct WithComplexPhantomData<T> {
    value: u32,
    _phantom: PhantomData<T>,
}

#[test]
fn test_phantom_data_with_custom_type() {
    let s1 = WithComplexPhantomData::<ComplexType> {
        value: 10,
        _phantom: PhantomData,
    };
    let s2 = WithComplexPhantomData::<ComplexType> {
        value: 10,
        _phantom: PhantomData,
    };
    assert_eq!(s1.ehash(), s2.ehash());
}

#[test]
fn test_phantom_data_with_vec_type() {
    let s1 = WithComplexPhantomData::<Vec<u32>> {
        value: 10,
        _phantom: PhantomData,
    };
    let s2 = WithComplexPhantomData::<Vec<u32>> {
        value: 10,
        _phantom: PhantomData,
    };
    assert_eq!(s1.ehash(), s2.ehash());
}

#[test]
fn test_phantom_data_different_vec_element_types() {
    let s1 = WithComplexPhantomData::<Vec<u32>> {
        value: 10,
        _phantom: PhantomData,
    };
    let s2 = WithComplexPhantomData::<Vec<u64>> {
        value: 10,
        _phantom: PhantomData,
    };
    assert_ne!(s1.ehash(), s2.ehash());
}
