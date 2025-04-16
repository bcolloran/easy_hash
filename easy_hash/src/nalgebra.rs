use nalgebra::{Const, OPoint, UnitVector2};

use crate::{type_salt, EasyHash};

impl EasyHash for nalgebra::Vector2<f32> {
    const TYPE_SALT: u32 = type_salt::<Self>();
    fn ehash(&self) -> u64 {
        let mut checksum = fletcher::Fletcher64::new();
        checksum.update(&[Self::TYPE_SALT, self.x.to_bits(), self.y.to_bits()]);
        checksum.value()
    }
}

impl EasyHash for nalgebra::Vector3<f32> {
    const TYPE_SALT: u32 = type_salt::<Self>();
    fn ehash(&self) -> u64 {
        let mut checksum = fletcher::Fletcher64::new();
        checksum.update(&[
            Self::TYPE_SALT,
            self.x.to_bits(),
            self.y.to_bits(),
            self.z.to_bits(),
        ]);
        checksum.value()
    }
}

impl EasyHash for UnitVector2<f32> {
    const TYPE_SALT: u32 = type_salt::<Self>();
    fn ehash(&self) -> u64 {
        let mut checksum = fletcher::Fletcher64::new();
        checksum.update(&[Self::TYPE_SALT, self.x.to_bits(), self.y.to_bits()]);
        checksum.value()
    }
}

impl EasyHash for OPoint<f32, Const<2>> {
    const TYPE_SALT: u32 = type_salt::<Self>();
    fn ehash(&self) -> u64 {
        let mut checksum = fletcher::Fletcher64::new();
        checksum.update(&[Self::TYPE_SALT, self.x.to_bits(), self.y.to_bits()]);
        checksum.value()
    }
}
