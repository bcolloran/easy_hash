use crate::{split_u64, type_salt, EasyHash};

impl EasyHash for ordered_float::OrderedFloat<f32> {
    const TYPE_SALT: u32 = type_salt::<Self>();
    fn ehash(&self) -> u64 {
        let mut checksum = fletcher::Fletcher64::new();
        checksum.update(&[Self::TYPE_SALT, self.0.to_bits() as u32]);
        checksum.value()
    }
}
impl EasyHash for ordered_float::OrderedFloat<f64> {
    const TYPE_SALT: u32 = type_salt::<Self>();
    fn ehash(&self) -> u64 {
        let mut checksum = fletcher::Fletcher64::new();
        checksum.update(&[Self::TYPE_SALT]);
        let bits = self.to_bits() as u64;
        checksum.update(&split_u64(bits));
        checksum.value()
    }
}

impl EasyHash for ordered_float::NotNan<f32> {
    const TYPE_SALT: u32 = type_salt::<Self>();
    fn ehash(&self) -> u64 {
        let mut checksum = fletcher::Fletcher64::new();
        checksum.update(&[Self::TYPE_SALT, self.to_bits() as u32]);
        checksum.value()
    }
}

impl EasyHash for ordered_float::NotNan<f64> {
    const TYPE_SALT: u32 = type_salt::<Self>();
    fn ehash(&self) -> u64 {
        let mut checksum = fletcher::Fletcher64::new();
        checksum.update(&[Self::TYPE_SALT]);
        let bits = self.to_bits() as u64;
        checksum.update(&split_u64(bits));
        checksum.value()
    }
}
