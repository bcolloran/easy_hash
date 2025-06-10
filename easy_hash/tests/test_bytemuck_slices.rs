use easy_hash::EasyHash;

#[test]
fn test_slice_u32() {
    let data = [1u32, 2, 3, 4, 5];
    let slice: &[u32] = &data;
    let hash = slice.ehash();

    // Should be deterministic
    assert_eq!(hash, slice.ehash());

    // Different data should produce different hash
    let data2 = [1u32, 2, 3, 4, 6];
    let slice2: &[u32] = &data2;
    assert_ne!(hash, slice2.ehash());
}

#[test]
fn test_slice_u64() {
    let data = [100u64, 200, 300, 400, 500];
    let slice: &[u64] = &data;
    let hash = slice.ehash();

    // Should be deterministic
    assert_eq!(hash, slice.ehash());

    // Different data should produce different hash
    let data2 = [100u64, 200, 300, 400, 501];
    let slice2: &[u64] = &data2;
    assert_ne!(hash, slice2.ehash());
}

// #[test]
// fn test_slice_u8() {
//     let data = [0x01u8, 0x02, 0x03, 0x04, 0x05];
//     let slice: &[u8] = &data;
//     let hash = slice.ehash();

//     // Should be deterministic
//     assert_eq!(hash, slice.ehash());

//     // Different order should produce different hash
//     let data2 = [0x02u8, 0x01, 0x03, 0x04, 0x05];
//     let slice2: &[u8] = &data2;
//     assert_ne!(hash, slice2.ehash());
// }

#[test]
fn test_slice_i32() {
    let data = [-1i32, 0, 1, 42, 100];
    let slice: &[i32] = &data;
    let hash = slice.ehash();

    // Should be deterministic
    assert_eq!(hash, slice.ehash());

    // Different values should produce different hash
    let data2 = [-1i32, 0, 1, -42, 100];
    let slice2: &[i32] = &data2;
    assert_ne!(hash, slice2.ehash());
}

#[test]
fn test_empty_slice() {
    let empty_u32: &[u32] = &[];
    let empty_u64: &[u64] = &[];

    // Empty slices should hash consistently
    assert_eq!(empty_u32.ehash(), empty_u32.ehash());

    // But different types should hash differently
    assert_ne!(empty_u32.ehash(), empty_u64.ehash());
}

#[test]
fn test_fixed_array_u32() {
    let array: [u32; 5] = [10, 20, 30, 40, 50];
    let hash = array.ehash();

    // Should be deterministic
    assert_eq!(hash, array.ehash());

    // Different array should produce different hash
    let array2: [u32; 5] = [10, 20, 30, 40, 51];
    assert_ne!(hash, array2.ehash());
}

#[test]
fn test_fixed_array_u64() {
    let array: [u64; 5] = [1000, 2000, 3000, 4000, 5000];
    let hash = array.ehash();

    // Should be deterministic
    assert_eq!(hash, array.ehash());

    // Same values but different size should hash differently
    let array2: [u64; 4] = [1000, 2000, 3000, 4000];
    assert_ne!(hash, array2.ehash());
}

// #[test]
// fn test_fixed_array_u8() {
//     let array: [u8; 5] = [1, 2, 3, 4, 5];
//     let hash = array.ehash();

//     // Should be deterministic
//     assert_eq!(hash, array.ehash());

//     // Reversed array should hash differently
//     let array2: [u8; 5] = [5, 4, 3, 2, 1];
//     assert_ne!(hash, array2.ehash());
// }

// #[test]
// fn test_fixed_array_i16() {
//     let array: [i16; 5] = [-100, -50, 0, 50, 100];
//     let hash = array.ehash();

//     // Should be deterministic
//     assert_eq!(hash, array.ehash());

//     // Different values should produce different hash
//     let array2: [i16; 5] = [-100, -50, 0, 50, 101];
//     assert_ne!(hash, array2.ehash());
// }

#[test]
fn test_array_vs_slice_different_types() {
    let data = [1u32, 2, 3, 4, 5];
    let array_hash = data.ehash();
    let slice_hash = data.as_slice().ehash();

    // Array and slice of same data should hash differently due to type salt
    assert_ne!(array_hash, slice_hash);
}

#[test]
fn test_different_array_sizes_same_data() {
    let array3: [u32; 3] = [1, 2, 3];
    let array4: [u32; 4] = [1, 2, 3, 0];

    // Different sized arrays should hash differently even with similar data
    assert_ne!(array3.ehash(), array4.ehash());
}

#[test]
fn test_single_element_arrays() {
    let array1: [u32; 1] = [42];
    let array2: [u32; 1] = [43];

    // Single element arrays should work correctly
    assert_eq!(array1.ehash(), array1.ehash());
    assert_ne!(array1.ehash(), array2.ehash());
}

#[test]
fn test_large_array() {
    let array: [u32; 100] = [42; 100];
    let hash = array.ehash();

    // Large arrays should hash consistently
    assert_eq!(hash, array.ehash());

    // Modify one element and hash should change
    let mut array2 = [42u32; 100];
    array2[50] = 43;
    assert_ne!(hash, array2.ehash());
}

// #[test]
// fn test_type_salt_consistency() {
//     // Test that type salts are working correctly by ensuring
//     // different types with same binary representation hash differently
//     let u32_data: [u32; 2] = [0x12345678, 0x9ABCDEF0];
//     let u16_data: [u16; 4] = [0x5678, 0x1234, 0xDEF0, 0x9ABC];

//     // These have the same binary representation but different types
//     // so they should hash differently due to type salt
//     assert_ne!(u32_data.ehash(), u16_data.ehash());
// }
