use easy_hash::EasyHash;

#[test]
fn test_equal_type_ids() {
    let t1 = std::any::TypeId::of::<u32>();
    let t2 = std::any::TypeId::of::<u32>();
    assert_eq!(t1.ehash(), t2.ehash());
}

#[test]
fn test_several_non_equal_type_ids() {
    let type_ids = [
        std::any::TypeId::of::<u32>(),
        std::any::TypeId::of::<u64>(),
        std::any::TypeId::of::<f32>(),
        std::any::TypeId::of::<f64>(),
        std::any::TypeId::of::<bool>(),
        std::any::TypeId::of::<String>(),
        std::any::TypeId::of::<()>(),
        std::any::TypeId::of::<(u32, u32)>(),
        std::any::TypeId::of::<Option<(u32, u32)>>(),
    ];

    let mut hashes: Vec<u64> = type_ids.iter().map(|id| id.ehash()).collect();
    hashes.sort_unstable();
    hashes.dedup();
    assert_eq!(hashes.len(), type_ids.len());
}
