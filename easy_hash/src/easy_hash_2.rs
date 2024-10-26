pub use easy_hash_derive::*;

use super::type_salt;

pub trait EasyHashTypeSalt {
    const TYPE_SALT_2: u32;
}

impl<T> EasyHashTypeSalt for T {
    const TYPE_SALT_2: u32 = type_salt::<T>();
}

pub trait EasyHash2 {
    fn ehash(&self) -> u64;
}

pub trait ToU32Slices {
    fn to_u32_stream(&self) -> Vec<&[u32]>;
}

impl<T> EasyHash2 for T
where
    T: EasyHashTypeSalt + ToU32Slices,
{
    fn ehash(&self) -> u64 {
        let mut checksum = fletcher::Fletcher64::new();
        checksum.update(&[Self::TYPE_SALT_2]);
        for slice in self.to_u32_stream() {
            checksum.update(slice);
        }
        // checksum.update(self.to_u32_stream());
        checksum.value()
    }
}

impl<T, U> ToU32Slices for (T, U)
where
    T: ToU32Slices,
    U: ToU32Slices,
{
    fn to_u32_stream(&self) -> Vec<&[u32]> {
        let mut stream = Vec::new();
        stream.extend(self.0.to_u32_stream());
        stream.extend(self.1.to_u32_stream());
        stream
    }
}
