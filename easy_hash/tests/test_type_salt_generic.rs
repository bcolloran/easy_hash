use easy_hash::type_salt_generic;

#[test]
fn test_type_salt_generic_deterministic() {
    let first = type_salt_generic::<Option<u8>, u16>();
    let second = type_salt_generic::<Option<u8>, u16>();

    assert_eq!(first, second);
}

#[test]
fn test_type_salt_generic_differs_for_generic() {
    let baseline = type_salt_generic::<Option<u8>, u16>();
    let different_generic = type_salt_generic::<Option<u8>, u32>();
    let swapped = type_salt_generic::<u16, Option<u8>>();

    assert_ne!(baseline, different_generic);
    assert_ne!(baseline, swapped);
}
