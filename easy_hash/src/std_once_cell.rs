use std::cell::OnceCell;

use crate::{EasyHash, split_u64, type_salt};
use fletcher::calc_fletcher64;

impl<T> EasyHash for OnceCell<T>
where
    T: EasyHash,
{
    const TYPE_SALT: u32 = type_salt::<&T>();

    fn ehash(&self) -> u64 {
        const NONE_VAL: u32 = 961_157_112;

        if let Some(x) = self.get() {
            let parts = split_u64(x.ehash());
            calc_fletcher64(&[Self::TYPE_SALT, parts[0], parts[1]])
        } else {
            calc_fletcher64(&[Self::TYPE_SALT, NONE_VAL])
        }
    }
}
