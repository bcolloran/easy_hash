use crate::{EasyHash, type_salt};
use bytemuck::{Pod, cast_slice, pod_align_to};
use fletcher::Fletcher64;

/// Update the checksum with at most 4 bytes of `data`,
/// padding with zeroes if needed.
fn update_unaligned<T: Pod>(checksum: &mut Fletcher64, data: &[T]) {
    if !data.is_empty() {
        let bytes: &[u8] = cast_slice::<T, u8>(data);
        debug_assert!(bytes.len() < 4);
        let mut buf = [0u8; 4];
        buf[..bytes.len()].copy_from_slice(bytes);
        checksum.update(&[u32::from_le_bytes(buf)]);
    }
}

impl<T, const N: usize> EasyHash for [T; N]
where
    T: Pod,
{
    const TYPE_SALT: u32 = type_salt::<[T; N]>();

    fn ehash(&self) -> u64 {
        let mut checksum = Fletcher64::new();
        checksum.update(&[Self::TYPE_SALT]);

        // split into head / aligned body / tail
        let (head, body_u32, tail) = pod_align_to::<T, u32>(self);

        update_unaligned(&mut checksum, head);
        checksum.update(body_u32);
        update_unaligned(&mut checksum, tail);

        checksum.value()
    }
}

impl<T> EasyHash for [T]
where
    T: Pod,
{
    const TYPE_SALT: u32 = type_salt::<&[T]>();

    fn ehash(&self) -> u64 {
        let mut checksum = Fletcher64::new();
        checksum.update(&[Self::TYPE_SALT]);

        let (head, body_u32, tail) = pod_align_to::<T, u32>(self);

        update_unaligned(&mut checksum, head);
        checksum.update(body_u32);
        update_unaligned(&mut checksum, tail);

        checksum.value()
    }
}
