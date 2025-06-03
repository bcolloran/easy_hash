use easy_hash::EasyHash;
use ordered_float::{OrderedFloat, NotNan};

#[test]
fn test_ordered_float_f32_hash() {
    let a = OrderedFloat(1.25f32);
    let b = OrderedFloat(1.25f32);
    let c = OrderedFloat(2.0f32);
    assert_eq!(a.ehash(), b.ehash());
    assert_ne!(a.ehash(), c.ehash());
    // same value as plain f32 should differ because of type salt
    assert_ne!(a.ehash(), (1.25f32).ehash());
}

#[test]
fn test_notnan_f32_hash() {
    let a = NotNan::new(3.5f32).unwrap();
    let b = NotNan::new(3.5f32).unwrap();
    let c = NotNan::new(4.0f32).unwrap();
    assert_eq!(a.ehash(), b.ehash());
    assert_ne!(a.ehash(), c.ehash());
}

#[test]
fn test_ordered_float_f64_hash() {
    let a = OrderedFloat(1.0f64);
    let b = OrderedFloat(1.0f64);
    let c = OrderedFloat(-1.0f64);
    assert_eq!(a.ehash(), b.ehash());
    assert_ne!(a.ehash(), c.ehash());
    assert_ne!(a.ehash(), (1.0f64).ehash());
}

#[test]
fn test_notnan_f64_hash() {
    let a = NotNan::new(0.0f64).unwrap();
    let b = NotNan::new(0.0f64).unwrap();
    let c = NotNan::new(1.0f64).unwrap();
    assert_eq!(a.ehash(), b.ehash());
    assert_ne!(a.ehash(), c.ehash());
}
