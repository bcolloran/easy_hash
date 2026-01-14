use crate::{EasyHash, calc_fletcher64, split_u64, type_salt};

impl EasyHash for bool {
    const TYPE_SALT: u32 = type_salt::<bool>();
    fn ehash(&self) -> u64 {
        calc_fletcher64(&[Self::TYPE_SALT, *self as u32])
    }
}

/// NOTE: for bit types, fletcher cannot differentiate between
///  binary all 0 and all 1, so in the case of all 1, we append
/// an extra copy of the TYPE_SALT to the checksum

impl EasyHash for u8 {
    const TYPE_SALT: u32 = type_salt::<u8>();

    fn ehash(&self) -> u64 {
        if *self == u8::MAX {
            calc_fletcher64(&[Self::TYPE_SALT, *self as u32 | Self::TYPE_SALT])
        } else {
            calc_fletcher64(&[Self::TYPE_SALT, *self as u32])
        }
    }
}

impl EasyHash for u16 {
    const TYPE_SALT: u32 = type_salt::<Self>();

    fn ehash(&self) -> u64 {
        if *self == u16::MAX {
            calc_fletcher64(&[Self::TYPE_SALT, *self as u32 | Self::TYPE_SALT])
        } else {
            calc_fletcher64(&[Self::TYPE_SALT, *self as u32])
        }
    }
}

impl EasyHash for u32 {
    const TYPE_SALT: u32 = type_salt::<Self>();

    fn ehash(&self) -> u64 {
        if *self == u32::MAX {
            calc_fletcher64(&[Self::TYPE_SALT, *self, Self::TYPE_SALT])
        } else {
            calc_fletcher64(&[Self::TYPE_SALT, *self])
        }
    }
}

impl EasyHash for u64 {
    const TYPE_SALT: u32 = type_salt::<Self>();

    fn ehash(&self) -> u64 {
        let mut checksum = fletcher::Fletcher64::new();
        checksum.update(&[Self::TYPE_SALT]);
        checksum.update(&split_u64(*self));
        if *self == u64::MAX {
            checksum.update(&[Self::TYPE_SALT]);
        }
        checksum.value()
    }
}

impl EasyHash for usize {
    const TYPE_SALT: u32 = type_salt::<Self>();

    fn ehash(&self) -> u64 {
        let mut checksum = fletcher::Fletcher64::new();
        checksum.update(&[Self::TYPE_SALT]);
        checksum.update(&split_u64(*self as u64));
        if *self as u64 == u64::MAX {
            checksum.update(&[Self::TYPE_SALT]);
        }
        checksum.value()
    }
}

impl EasyHash for i8 {
    const TYPE_SALT: u32 = type_salt::<i8>();

    fn ehash(&self) -> u64 {
        if *self as u8 == u8::MAX {
            calc_fletcher64(&[Self::TYPE_SALT, *self as u32 | Self::TYPE_SALT])
        } else {
            calc_fletcher64(&[Self::TYPE_SALT, *self as u32])
        }
    }
}

impl EasyHash for i16 {
    const TYPE_SALT: u32 = type_salt::<i16>();

    fn ehash(&self) -> u64 {
        if *self as u16 == u16::MAX {
            calc_fletcher64(&[Self::TYPE_SALT, *self as u32 | Self::TYPE_SALT])
        } else {
            calc_fletcher64(&[Self::TYPE_SALT, *self as u32])
        }
    }
}

impl EasyHash for i32 {
    const TYPE_SALT: u32 = type_salt::<i32>();

    fn ehash(&self) -> u64 {
        if *self as u32 == u32::MAX {
            calc_fletcher64(&[Self::TYPE_SALT, *self as u32, Self::TYPE_SALT])
        } else {
            calc_fletcher64(&[Self::TYPE_SALT, *self as u32])
        }
    }
}

impl EasyHash for i64 {
    const TYPE_SALT: u32 = type_salt::<i64>();

    fn ehash(&self) -> u64 {
        let mut checksum = fletcher::Fletcher64::new();
        checksum.update(&[Self::TYPE_SALT]);
        checksum.update(&split_u64(*self as u64));
        if *self as u64 == u64::MAX {
            checksum.update(&[Self::TYPE_SALT]);
        }
        checksum.value()
    }
}

impl EasyHash for isize {
    const TYPE_SALT: u32 = type_salt::<isize>();

    fn ehash(&self) -> u64 {
        let mut checksum = fletcher::Fletcher64::new();
        checksum.update(&[Self::TYPE_SALT]);
        checksum.update(&split_u64(*self as u64));
        if *self as u64 == u64::MAX {
            checksum.update(&[Self::TYPE_SALT]);
        }
        checksum.value()
    }
}

impl EasyHash for f32 {
    const TYPE_SALT: u32 = type_salt::<f32>();
    fn ehash(&self) -> u64 {
        let bits = self.to_bits() as u32;
        calc_fletcher64(&[f32::TYPE_SALT, bits])
    }
}

impl EasyHash for f64 {
    const TYPE_SALT: u32 = type_salt::<f32>();
    fn ehash(&self) -> u64 {
        let mut checksum = fletcher::Fletcher64::new();
        checksum.update(&[Self::TYPE_SALT]);
        let bits = self.to_bits() as u64;
        checksum.update(&split_u64(bits));
        checksum.value()
    }
}
