use crate::{EasyHash, type_salt};

use godot::builtin::Vector3;

impl EasyHash for Vector3 {
    const TYPE_SALT: u32 = type_salt::<Self>();
    fn ehash(&self) -> u64 {
        let mut checksum = fletcher::Fletcher64::new();
        checksum.update(&[
            Self::TYPE_SALT,
            self.x.to_bits() as u32,
            self.y.to_bits() as u32,
            self.z.to_bits() as u32,
        ]);
        checksum.value()
    }
}

impl EasyHash for godot::builtin::Vector2 {
    const TYPE_SALT: u32 = type_salt::<Self>();
    fn ehash(&self) -> u64 {
        let mut checksum = fletcher::Fletcher64::new();
        checksum.update(&[
            Self::TYPE_SALT,
            self.x.to_bits() as u32,
            self.y.to_bits() as u32,
        ]);

        checksum.value()
    }
}

impl EasyHash for godot::builtin::Vector3i {
    const TYPE_SALT: u32 = type_salt::<Self>();
    fn ehash(&self) -> u64 {
        let mut checksum = fletcher::Fletcher64::new();
        checksum.update(&[Self::TYPE_SALT, self.x as u32, self.y as u32, self.z as u32]);
        checksum.value()
    }
}

impl EasyHash for godot::builtin::Vector2i {
    const TYPE_SALT: u32 = type_salt::<Self>();
    fn ehash(&self) -> u64 {
        let mut checksum = fletcher::Fletcher64::new();
        checksum.update(&[Self::TYPE_SALT, self.x as u32, self.y as u32]);
        checksum.value()
    }
}
