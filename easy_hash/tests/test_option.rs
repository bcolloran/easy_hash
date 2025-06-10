use easy_hash::EasyHash;

#[test]
fn test_option_some_vs_none() {
    let some = Some(42u32);
    let none: Option<u32> = None;
    assert_ne!(some.ehash(), none.ehash());
}

#[test]
fn test_option_equality() {
    let a: Option<u64> = Some(5);
    let b: Option<u64> = Some(5);
    let c: Option<u64> = None;
    let d: Option<u64> = None;
    assert_eq!(a.ehash(), b.ehash());
    assert_eq!(c.ehash(), d.ehash());
    assert_ne!(a.ehash(), c.ehash());
}
