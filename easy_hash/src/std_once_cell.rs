use std::cell::OnceCell;

use crate::{split_u64, type_salt, EasyHash};

impl<T> EasyHash for OnceCell<T>
where
    T: EasyHash,
{
    const TYPE_SALT: u32 = type_salt::<&T>();

    fn ehash(&self) -> u64 {
        let mut checksum = fletcher::Fletcher64::new();
        checksum.update(&[Self::TYPE_SALT]);

        if let Some(x) = self.get() {
            checksum.update(&split_u64(x.ehash()));
        } else {
            let none_val: u32 = 961157112;
            checksum.update(&[none_val]);
        }

        checksum.value()
    }
}
