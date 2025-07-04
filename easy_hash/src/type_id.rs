use std::{
    any::TypeId,
    hash::{Hash, Hasher},
};

use crate::{EasyHash, type_salt};

struct IdentityHasher(u64);

impl Hasher for IdentityHasher {
    #[inline]
    fn write(&mut self, _bytes: &[u8]) {
        unimplemented!(
            "IdentityHasher does not support writing bytes directly. Use write_u64 instead."
        );
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        // IdentityHasher just stores the value directly, so we don't need to do anything here.
        self.0 = i;
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }
}

impl EasyHash for TypeId {
    const TYPE_SALT: u32 = type_salt::<Self>();
    // NOTE: EasyHash for TypeId just passes through the already-hashed value for the type. Thus, the TYPE_SALT is irrelevant, and so not passed through to final hash value
    fn ehash(&self) -> u64 {
        let mut hasher = IdentityHasher(0);
        self.hash(&mut hasher);
        hasher.finish()
    }
}
