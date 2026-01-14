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
        // calc_fletcher64(&[u16::TYPE_SALT, *self as u32])
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
        // calc_fletcher64(&[u32::TYPE_SALT, *self as u32])
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
        // calc_fletcher64(&[usize::TYPE_SALT, *self as u32])
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
        // calc_fletcher64(&[i16::TYPE_SALT, *self as u32])
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
        // calc_fletcher64(&[u32::TYPE_SALT, *self as u32])
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
        // calc_fletcher64(&[usize::TYPE_SALT, *self as u32])
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
