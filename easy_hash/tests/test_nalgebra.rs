use easy_hash::EasyHash;
use nalgebra::{Isometry2, Point2, UnitVector2, Vector2, Vector3};

#[test]
fn test_vector2_hash() {
    let a = Vector2::new(1.0f32, 2.0);
    let b = Vector2::new(1.0f32, 2.0);
    let c = Vector2::new(2.0f32, 3.0);
    assert_eq!(a.ehash(), b.ehash());
    assert_ne!(a.ehash(), c.ehash());
}

#[test]
fn test_vector3_hash() {
    let a = Vector3::new(1.0f32, 2.0, 3.0);
    let b = Vector3::new(1.0f32, 2.0, 3.0);
    let c = Vector3::new(3.0f32, 2.0, 1.0);
    assert_eq!(a.ehash(), b.ehash());
    assert_ne!(a.ehash(), c.ehash());
}

#[test]
fn test_unit_vector2_hash() {
    let a = UnitVector2::new_normalize(Vector2::new(1.0, 0.0));
    let b = UnitVector2::new_normalize(Vector2::new(1.0, 0.0));
    let c = UnitVector2::new_normalize(Vector2::new(0.0, 1.0));
    assert_eq!(a.ehash(), b.ehash());
    assert_ne!(a.ehash(), c.ehash());
}

#[test]
fn test_point2_hash() {
    let a: Point2<f32> = Point2::new(1.0, 2.0);
    let b: Point2<f32> = Point2::new(1.0, 2.0);
    let c: Point2<f32> = Point2::new(2.0, 3.0);
    assert_eq!(a.ehash(), b.ehash());
    assert_ne!(a.ehash(), c.ehash());
}

#[test]
fn test_isometry2_hash() {
    let a: Isometry2<f32> = Isometry2::new(Vector2::new(1.0, 2.0), 0.0);
    let b: Isometry2<f32> = Isometry2::new(Vector2::new(1.0, 2.0), 0.0);
    let c: Isometry2<f32> = Isometry2::new(Vector2::new(2.0, 3.0), 0.0);
    let d: Isometry2<f32> = Isometry2::new(Vector2::new(1.0, 2.0), std::f32::consts::PI / 4.0);

    assert_eq!(a.ehash(), b.ehash());
    assert_ne!(a.ehash(), c.ehash());
    assert_ne!(a.ehash(), d.ehash());
}
