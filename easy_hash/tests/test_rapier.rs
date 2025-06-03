use easy_hash::EasyHash;
use rapier2d::prelude::{RigidBodyHandle, ColliderHandle, ImpulseJointHandle, MultibodyJointHandle};

#[test]
fn test_rigid_body_handle_hash() {
    let a = RigidBodyHandle::from_raw_parts(1, 2);
    let b = RigidBodyHandle::from_raw_parts(1, 2);
    let c = RigidBodyHandle::from_raw_parts(2, 2);
    assert_eq!(a.ehash(), b.ehash());
    assert_ne!(a.ehash(), c.ehash());
}

#[test]
fn test_collider_handle_hash() {
    let a = ColliderHandle::from_raw_parts(3, 4);
    let b = ColliderHandle::from_raw_parts(3, 4);
    let c = ColliderHandle::from_raw_parts(3, 5);
    assert_eq!(a.ehash(), b.ehash());
    assert_ne!(a.ehash(), c.ehash());
}

#[test]
fn test_impulse_joint_handle_hash() {
    let a = ImpulseJointHandle::from_raw_parts(7, 8);
    let b = ImpulseJointHandle::from_raw_parts(7, 8);
    let c = ImpulseJointHandle::from_raw_parts(8, 8);
    assert_eq!(a.ehash(), b.ehash());
    assert_ne!(a.ehash(), c.ehash());
}

#[test]
fn test_multibody_joint_handle_hash() {
    let a = MultibodyJointHandle::from_raw_parts(9, 10);
    let b = MultibodyJointHandle::from_raw_parts(9, 10);
    let c = MultibodyJointHandle::from_raw_parts(11, 10);
    assert_eq!(a.ehash(), b.ehash());
    assert_ne!(a.ehash(), c.ehash());
}
