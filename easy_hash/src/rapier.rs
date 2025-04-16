use rapier2d::prelude::{
    ColliderHandle, ImpulseJointHandle, MultibodyJointHandle, RigidBodyHandle,
};

use crate::{type_salt, EasyHash};

impl EasyHash for RigidBodyHandle {
    const TYPE_SALT: u32 = type_salt::<Self>();
    fn ehash(&self) -> u64 {
        let mut checksum = fletcher::Fletcher64::new();
        let raw_parts = self.into_raw_parts();
        checksum.update(&[Self::TYPE_SALT, raw_parts.0, raw_parts.1]);
        checksum.value()
    }
}

impl EasyHash for ColliderHandle {
    const TYPE_SALT: u32 = type_salt::<Self>();
    fn ehash(&self) -> u64 {
        let mut checksum = fletcher::Fletcher64::new();
        let raw_parts = self.into_raw_parts();
        checksum.update(&[Self::TYPE_SALT, raw_parts.0, raw_parts.1]);
        checksum.value()
    }
}

impl EasyHash for ImpulseJointHandle {
    const TYPE_SALT: u32 = type_salt::<Self>();
    fn ehash(&self) -> u64 {
        let mut checksum = fletcher::Fletcher64::new();
        let raw_parts = self.into_raw_parts();
        checksum.update(&[Self::TYPE_SALT, raw_parts.0, raw_parts.1]);
        checksum.value()
    }
}

impl EasyHash for MultibodyJointHandle {
    const TYPE_SALT: u32 = type_salt::<Self>();
    fn ehash(&self) -> u64 {
        let mut checksum = fletcher::Fletcher64::new();
        let raw_parts = self.into_raw_parts();
        checksum.update(&[Self::TYPE_SALT, raw_parts.0, raw_parts.1]);
        checksum.value()
    }
}
