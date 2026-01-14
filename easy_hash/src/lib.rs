#![doc = include_str!("../../README.md")]
#![feature(const_type_name)]

use const_fnv1a_hash::fnv1a_hash_str_32;

#[cfg(feature = "godot")]
pub mod godot;
#[cfg(feature = "nalgebra")]
pub mod nalgebra;
#[cfg(feature = "ordered_float")]
pub mod ordered_float;
#[cfg(feature = "rapier")]
pub mod rapier;
#[cfg(feature = "bevy")]
use bevy_ecs::prelude::Mut;
#[cfg(feature = "bevy")]
impl<T> EasyHash for Mut<'_, T>
where
    T: EasyHash,
{
    const TYPE_SALT: u32 = type_salt::<&T>();

    fn ehash(&self) -> u64 {
        (**self).ehash()
    }
}

pub use easy_hash_derive::*;
pub use fletcher;

use fletcher::*;

pub mod bytemuck_slices;
pub mod primitives;
pub mod std_once_cell;
pub mod tuples;
pub mod type_id;
pub mod type_salt_generic;
pub use type_salt_generic::*;

pub trait EasyHash {
    const TYPE_SALT: u32;
    fn ehash(&self) -> u64;
}

pub const fn type_salt<T>() -> u32 {
    fnv1a_hash_str_32(std::any::type_name::<T>())
}

#[inline]
pub fn split_u64(x: u64) -> [u32; 2] {
    [(x >> 32) as u32, x as u32]
}

#[inline]
pub fn u64_to_u32_slice(x: &[u64]) -> &[u32] {
    bytemuck::cast_slice(x)
}

#[inline]
pub fn join_u32s(a: u32, b: u32) -> u64 {
    ((a as u64) << 32) | (b as u64)
}

#[inline]
pub fn split_i64(x: i64) -> [u32; 2] {
    [(x >> 32) as u32, x as u32]
}

impl<T> EasyHash for &T
where
    T: EasyHash,
{
    const TYPE_SALT: u32 = type_salt::<&T>();

    fn ehash(&self) -> u64 {
        (**self).ehash()
    }
}

impl<T> EasyHash for std::marker::PhantomData<T> {
    const TYPE_SALT: u32 = type_salt::<std::marker::PhantomData<T>>();

    fn ehash(&self) -> u64 {
        // PhantomData has no runtime data, only type information
        // We hash only the type salt to differentiate between different PhantomData<T> types
        calc_fletcher64(&[Self::TYPE_SALT])
    }
}

impl<T> EasyHash for Option<T>
where
    T: EasyHash,
{
    const TYPE_SALT: u32 = type_salt::<T>();

    fn ehash(&self) -> u64 {
        const NONE_VAL: u32 = 780526312;

        if let Some(x) = self {
            let parts = split_u64(x.ehash());
            calc_fletcher64(&[Self::TYPE_SALT, parts[0], parts[1]])
        } else {
            calc_fletcher64(&[Self::TYPE_SALT, NONE_VAL])
        }
    }
}

impl<T> EasyHash for Vec<T>
where
    T: EasyHash,
{
    const TYPE_SALT: u32 = type_salt::<Vec<T>>();

    fn ehash(&self) -> u64 {
        let mut checksum = fletcher::Fletcher64::new();
        checksum.update(&[Self::TYPE_SALT]);
        let hashes: Vec<u64> = self.iter().map(|x| x.ehash()).collect();
        checksum.update(u64_to_u32_slice(&hashes));
        checksum.value()
    }
}

impl EasyHash for &str {
    const TYPE_SALT: u32 = type_salt::<Self>();
    fn ehash(&self) -> u64 {
        let mut checksum = fletcher::Fletcher64::new();
        checksum.update(&[Self::TYPE_SALT]);
        let bytes = self.as_bytes();
        let (chunks, remainder) = bytes.as_chunks::<4>();
        for chunk in chunks {
            checksum.update(&[u32::from_le_bytes(*chunk)]);
        }
        if !remainder.is_empty() {
            let mut byte = [0u8; 4];
            byte[..remainder.len()].copy_from_slice(remainder);
            checksum.update(&[u32::from_le_bytes(byte)]);
        }
        checksum.value()
    }
}

impl EasyHash for String {
    const TYPE_SALT: u32 = type_salt::<String>();
    fn ehash(&self) -> u64 {
        let mut checksum = fletcher::Fletcher64::new();
        checksum.update(&[Self::TYPE_SALT]);

        let bytes = self.as_bytes();
        let (chunks, remainder) = bytes.as_chunks::<4>();
        for chunk in chunks {
            checksum.update(&[u32::from_le_bytes(*chunk)]);
        }
        if !remainder.is_empty() {
            let mut byte = [0u8; 4];
            byte[..remainder.len()].copy_from_slice(remainder);
            checksum.update(&[u32::from_le_bytes(byte)]);
        }
        checksum.value()
    }
}
