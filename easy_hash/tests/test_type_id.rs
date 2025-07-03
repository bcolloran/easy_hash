use easy_hash::EasyHash;

#[test]
fn test_equal_type_ids() {
    let t1 = std::any::TypeId::of::<u32>();
    let t2 = std::any::TypeId::of::<u32>();
    assert_eq!(t1.ehash(), t2.ehash());
}

#[test]
fn test_several_non_equal_type_ids() {
    let t1 = std::any::TypeId::of::<u32>();
    let t2 = std::any::TypeId::of::<u64>();
    let t3 = std::any::TypeId::of::<f32>();
    let t4 = std::any::TypeId::of::<f64>();
    let t5 = std::any::TypeId::of::<bool>();
    let t6 = std::any::TypeId::of::<String>();
    let t7 = std::any::TypeId::of::<()>();
    let t8 = std::any::TypeId::of::<(u32,u32)>();
    let t9 = std::any::TypeId::of::<Option<(u32,u32)>>();

    assert_ne!(t1.ehash(), t2.ehash());
    assert_ne!(t1.ehash(), t3.ehash());
    assert_ne!(t1.ehash(), t4.ehash());
    assert_ne!(t2.ehash(), t3.ehash());
    assert_ne!(t2.ehash(), t4.ehash());
    assert_ne!(t3.ehash(), t4.ehash());
    assert_ne!(t1.ehash(), t5.ehash());
    assert_ne!(t1.ehash(), t6.ehash());
    assert_ne!(t1.ehash(), t7.ehash());
    assert_ne!(t1.ehash(), t8.ehash());
    assert_ne!(t1.ehash(), t9.ehash());
    assert_ne!(t2.ehash(), t5.ehash());
    assert_ne!(t2.ehash(), t6.ehash());
    assert_ne!(t2.ehash(), t7.ehash());
    assert_ne!(t2.ehash(), t8.ehash());
    assert_ne!(t2.ehash(), t9.ehash());
    assert_ne!(t3.ehash(), t5.ehash());
    assert_ne!(t3.ehash(), t6.ehash());
    assert_ne!(t3.ehash(), t7.ehash());
    assert_ne!(t3.ehash(), t8.ehash());
    assert_ne!(t3.ehash(), t9.ehash());
    assert_ne!(t4.ehash(), t5.ehash());
    assert_ne!(t4.ehash(), t6.ehash());
    assert_ne!(t4.ehash(), t7.ehash());
    assert_ne!(t4.ehash(), t8.ehash());
    assert_ne!(t4.ehash(), t9.ehash());
    assert_ne!(t5.ehash(), t6.ehash());
    assert_ne!(t5.ehash(), t7.ehash());
    assert_ne!(t5.ehash(), t8.ehash());
    assert_ne!(t5.ehash(), t9.ehash());
    assert_ne!(t6.ehash(), t7.ehash());
    assert_ne!(t6.ehash(), t8.ehash());
    assert_ne!(t6.ehash(), t9.ehash());
    assert_ne!(t7.ehash(), t8.ehash());
    assert_ne!(t7.ehash(), t9.ehash());
    assert_ne!(t8.ehash(), t9.ehash());
}
