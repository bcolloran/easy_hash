use bytemuck::pod_align_to;
use easy_hash::fletcher::Fletcher64;
use easy_hash::{EasyHash, type_salt};

fn hash_with_alignment<T: bytemuck::Pod>(data: &[T], type_salt: u32) -> u64 {
    let mut checksum = Fletcher64::new();
    checksum.update(&[type_salt]);

    let (head, body_u32, tail) = pod_align_to::<T, u32>(data);

    if !head.is_empty() {
        let bytes = bytemuck::cast_slice::<T, u8>(head);
        let mut buf = [0u8; 4];
        buf[..bytes.len()].copy_from_slice(bytes);
        checksum.update(&[u32::from_le_bytes(buf)]);
    }

    checksum.update(body_u32);

    if !tail.is_empty() {
        let bytes = bytemuck::cast_slice::<T, u8>(tail);
        let mut buf = [0u8; 4];
        buf[..bytes.len()].copy_from_slice(bytes);
        checksum.update(&[u32::from_le_bytes(buf)]);
    }

    checksum.value()
}

#[test]
fn test_u8_slice_unaligned_head_and_tail() {
    let data = [0x11u8, 0x22, 0x33, 0x44, 0x55];
    let slice = &data[1..];
    let expected = hash_with_alignment(slice, type_salt::<&[u8]>());

    assert_eq!(slice.ehash(), expected);
}

#[test]
fn test_u8_array_tail_padding() {
    let data = [0x10u8, 0x20, 0x30];
    let expected = hash_with_alignment(&data, type_salt::<[u8; 3]>());

    assert_eq!(data.ehash(), expected);
}
