use easy_hash::fletcher::Fletcher64;
use easy_hash::{EasyHash, type_salt};
use godot::builtin::{Vector2, Vector2i, Vector3, Vector3i};

#[test]
fn test_vector2_hash_matches_manual() {
    let value = Vector2::new(1.25, -2.5);
    let mut checksum = Fletcher64::new();
    checksum.update(&[type_salt::<Vector2>(), value.x.to_bits(), value.y.to_bits()]);

    assert_eq!(value.ehash(), checksum.value());
}

#[test]
fn test_vector3_hash_matches_manual() {
    let value = Vector3::new(1.25, -2.5, 3.75);
    let mut checksum = Fletcher64::new();
    checksum.update(&[
        type_salt::<Vector3>(),
        value.x.to_bits(),
        value.y.to_bits(),
        value.z.to_bits(),
    ]);

    assert_eq!(value.ehash(), checksum.value());
}

#[test]
fn test_vector2i_hash_matches_manual() {
    let value = Vector2i::new(7, -11);
    let mut checksum = Fletcher64::new();
    checksum.update(&[type_salt::<Vector2i>(), value.x as u32, value.y as u32]);

    assert_eq!(value.ehash(), checksum.value());
}

#[test]
fn test_vector3i_hash_matches_manual() {
    let value = Vector3i::new(7, -11, 42);
    let mut checksum = Fletcher64::new();
    checksum.update(&[
        type_salt::<Vector3i>(),
        value.x as u32,
        value.y as u32,
        value.z as u32,
    ]);

    assert_eq!(value.ehash(), checksum.value());
}
