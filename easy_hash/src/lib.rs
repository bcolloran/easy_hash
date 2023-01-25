#![feature(const_type_name)]

use std::ops::Range;

use bevy::prelude::Mut;
pub use easy_hash_derive::*;
use sha2_const::Sha256;

// use std::num::Wrapping;

use fletcher::*;

// pub struct EHashSummer {
//     checksum: fletcher::Fletcher64,
// }
// impl EHashSummer {
//     pub fn new(&self) {
//         EHashSummer {
//             checksum: fletcher::Fletcher64::new(),
//         }
//     }

//     pub fn update(&self, x: dyn EasyHash) {
//         self.checksum.update(&split_u64(x.ehash()));
//     }
// }

// fn ehash(&self) -> u64 {
//     let mut checksum = fletcher::Fletcher64::new();
//     checksum.update(&[Self::TYPE_SALT]);
//     for x in self {
//         checksum.update(&split_u64(x.ehash()));
//     }
//     checksum.value()
// }

pub trait EasyHash {
    const TYPE_SALT: u32;
    fn ehash(&self) -> u64;
}

// impl<T> EasyHash for T
// where
//     T: Hash,
// {
// //     fn ehash(&self) -> u64 {
//         let mut s = DefaultHasher::new();
//         std::any::type_name::<T>().hash(&mut s); //salt
//         self.hash(&mut s);
//         std::hash::Hasher::finish(&s)
//     }
// }

// trait SplitU64 {
//     fn split_u64(self) -> [u32; 2];
// }

// impl SplitU64 for u64 {
//     fn split_u64(self) -> [u32; 2] {
//         [self as u32, (self >> 32) as u32]
//     }
// }

pub const fn type_salt<T>() -> u32 {
    let hash = Sha256::new()
        .update(std::any::type_name::<T>().as_bytes())
        .finalize();
    u32::from_be_bytes([hash[0], hash[1], hash[2], hash[3]])
}

pub fn split_u64(x: u64) -> [u32; 2] {
    [x as u32, (x >> 32) as u32]
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
    const TYPE_SALT: u32 = type_salt::<&T>();

    fn ehash(&self) -> u64 {
        let mut checksum = fletcher::Fletcher64::new();
        checksum.update(&[Self::TYPE_SALT]);

        if let Some(x) = self {
            checksum.update(&split_u64(x.ehash()));
        } else {
            let none_val: u32 = 780526312;
            checksum.update(&[none_val]);
        }

        checksum.value()
    }
}

impl<T> EasyHash for Mut<'_, T>
where
    T: EasyHash,
{
    const TYPE_SALT: u32 = type_salt::<&T>();

    fn ehash(&self) -> u64 {
        (**self).ehash()
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
        for x in self {
            checksum.update(&split_u64(x.ehash()));
        }
        checksum.value()
    }
}

impl EasyHash for String {
    const TYPE_SALT: u32 = type_salt::<String>();
    fn ehash(&self) -> u64 {
        let mut checksum = fletcher::Fletcher64::new();
        checksum.update(&[Self::TYPE_SALT]);

        for chunk in self.as_bytes().rchunks(4) {
            let mut byte = [0u8; 4];
            for j in 0..4 {
                byte[j] = *chunk.get(j).unwrap_or(&0);
            }
            checksum.update(&[u32::from_le_bytes(byte)]);
        }
        checksum.value()
    }
}

impl EasyHash for bool {
    const TYPE_SALT: u32 = type_salt::<bool>();
    fn ehash(&self) -> u64 {
        // checksum.value()
        calc_fletcher64(&[bool::TYPE_SALT, *self as u32])
    }
}

impl EasyHash for u8 {
    const TYPE_SALT: u32 = type_salt::<u8>();

    fn ehash(&self) -> u64 {
        // (*self as u64) ^ 23452367569865902
        calc_fletcher64(&[u8::TYPE_SALT, *self as u32])
    }
}

impl EasyHash for u16 {
    const TYPE_SALT: u32 = type_salt::<u16>();

    fn ehash(&self) -> u64 {
        // (*self as u64) ^ 9218759616293562
        calc_fletcher64(&[u16::TYPE_SALT, *self as u32])
    }
}

impl EasyHash for u32 {
    const TYPE_SALT: u32 = type_salt::<u32>();

    fn ehash(&self) -> u64 {
        // (*self as u64) ^ 65736917917127009
        calc_fletcher64(&[u32::TYPE_SALT, *self as u32])
    }
}

impl EasyHash for u64 {
    const TYPE_SALT: u32 = type_salt::<u64>();

    fn ehash(&self) -> u64 {
        let mut checksum = fletcher::Fletcher64::new();
        checksum.update(&[Self::TYPE_SALT]);
        checksum.update(&split_u64(*self));
        checksum.value()
    }
}

impl EasyHash for usize {
    const TYPE_SALT: u32 = type_salt::<usize>();

    fn ehash(&self) -> u64 {
        // (*self as u64) ^ 65736917917127009
        calc_fletcher64(&[usize::TYPE_SALT, *self as u32])
    }
}

impl EasyHash for f32 {
    const TYPE_SALT: u32 = type_salt::<f32>();
    fn ehash(&self) -> u64 {
        let bits = self.to_bits() as u32;
        calc_fletcher64(&[f32::TYPE_SALT, bits])
    }
}

impl<A, B> EasyHash for (A, B)
where
    A: EasyHash,
    B: EasyHash,
{
    const TYPE_SALT: u32 = type_salt::<(A, B)>();

    fn ehash(&self) -> u64 {
        let mut checksum = fletcher::Fletcher64::new();
        checksum.update(&[Self::TYPE_SALT]);
        checksum.update(&split_u64(self.0.ehash()));
        checksum.update(&split_u64(self.1.ehash()));
        checksum.value()
    }
}

impl<A, B, C> EasyHash for (A, B, C)
where
    A: EasyHash,
    B: EasyHash,
    C: EasyHash,
{
    const TYPE_SALT: u32 = type_salt::<(A, B, C)>();

    fn ehash(&self) -> u64 {
        let mut checksum = fletcher::Fletcher64::new();
        checksum.update(&[Self::TYPE_SALT]);
        checksum.update(&split_u64(self.0.ehash()));
        checksum.update(&split_u64(self.1.ehash()));
        checksum.update(&split_u64(self.2.ehash()));
        checksum.value()
    }
}

impl<A, B, C, D> EasyHash for (A, B, C, D)
where
    A: EasyHash,
    B: EasyHash,
    C: EasyHash,
    D: EasyHash,
{
    const TYPE_SALT: u32 = type_salt::<Self>();

    fn ehash(&self) -> u64 {
        let mut checksum = fletcher::Fletcher64::new();

        checksum.update(&[Self::TYPE_SALT]);
        checksum.update(&split_u64(self.0.ehash()));
        checksum.update(&split_u64(self.1.ehash()));
        checksum.update(&split_u64(self.2.ehash()));
        checksum.update(&split_u64(self.3.ehash()));
        checksum.value()
    }
}

// impl<A, B, C, D> EasyHash for &(&A, &B, &C, &D)
// where
//     A: EasyHash,
//     B: EasyHash,
//     C: EasyHash,
//     D: EasyHash,
// {
//     fn ehash(&self) -> u64 {
//         (Wrapping(self.0.ehash())
//             + Wrapping(self.1.ehash())
//             + Wrapping(self.2.ehash())
//             + Wrapping(self.3.ehash()))
//         .0
//     }
// }

impl<A, B, C, D, E> EasyHash for (A, B, C, D, E)
where
    A: EasyHash,
    B: EasyHash,
    C: EasyHash,
    D: EasyHash,
    E: EasyHash,
{
    const TYPE_SALT: u32 = type_salt::<(A, B, C, D, E)>();
    fn ehash(&self) -> u64 {
        let mut checksum = fletcher::Fletcher64::new();
        checksum.update(&split_u64(self.0.ehash()));
        checksum.update(&split_u64(self.1.ehash()));
        checksum.update(&split_u64(self.2.ehash()));
        checksum.update(&split_u64(self.3.ehash()));
        checksum.update(&split_u64(self.4.ehash()));
        checksum.value()
    }
}
impl<A, B, C, D, E, F> EasyHash for (A, B, C, D, E, F)
where
    A: EasyHash,
    B: EasyHash,
    C: EasyHash,
    D: EasyHash,

    E: EasyHash,
    F: EasyHash,
{
    const TYPE_SALT: u32 = type_salt::<(A, B, C, D, E, F)>();
    fn ehash(&self) -> u64 {
        let mut checksum = fletcher::Fletcher64::new();
        checksum.update(&split_u64(self.0.ehash()));
        checksum.update(&split_u64(self.1.ehash()));
        checksum.update(&split_u64(self.2.ehash()));
        checksum.update(&split_u64(self.3.ehash()));
        checksum.update(&split_u64(self.4.ehash()));

        checksum.update(&split_u64(self.4.ehash()));
        checksum.update(&split_u64(self.5.ehash()));
        checksum.value()
    }
}
impl<A, B, C, D, E, F, G> EasyHash for (A, B, C, D, E, F, G)
where
    A: EasyHash,
    B: EasyHash,
    C: EasyHash,
    D: EasyHash,

    E: EasyHash,
    F: EasyHash,
    G: EasyHash,
{
    const TYPE_SALT: u32 = type_salt::<(A, B, C, D, E, F, G)>();
    fn ehash(&self) -> u64 {
        let mut checksum = fletcher::Fletcher64::new();
        checksum.update(&split_u64(self.0.ehash()));
        checksum.update(&split_u64(self.1.ehash()));
        checksum.update(&split_u64(self.2.ehash()));
        checksum.update(&split_u64(self.3.ehash()));
        checksum.update(&split_u64(self.4.ehash()));

        checksum.update(&split_u64(self.4.ehash()));
        checksum.update(&split_u64(self.5.ehash()));
        checksum.update(&split_u64(self.6.ehash()));
        checksum.value()
    }
}

impl<A, B, C, D, E, F, G, H> EasyHash for (A, B, C, D, E, F, G, H)
where
    A: EasyHash,
    B: EasyHash,
    C: EasyHash,
    D: EasyHash,

    E: EasyHash,
    F: EasyHash,
    G: EasyHash,
    H: EasyHash,
{
    const TYPE_SALT: u32 = type_salt::<(A, B, C, D, E, F, G, H)>();
    fn ehash(&self) -> u64 {
        let mut checksum = fletcher::Fletcher64::new();
        checksum.update(&split_u64(self.0.ehash()));
        checksum.update(&split_u64(self.1.ehash()));
        checksum.update(&split_u64(self.2.ehash()));
        checksum.update(&split_u64(self.3.ehash()));
        checksum.update(&split_u64(self.4.ehash()));

        checksum.update(&split_u64(self.4.ehash()));
        checksum.update(&split_u64(self.5.ehash()));
        checksum.update(&split_u64(self.6.ehash()));
        checksum.update(&split_u64(self.7.ehash()));
        checksum.value()
    }
}

impl<A, B, C, D, E, F, G, H, I> EasyHash for (A, B, C, D, E, F, G, H, I)
where
    A: EasyHash,
    B: EasyHash,
    C: EasyHash,
    D: EasyHash,

    E: EasyHash,
    F: EasyHash,
    G: EasyHash,
    H: EasyHash,

    I: EasyHash,
{
    const TYPE_SALT: u32 = type_salt::<(A, B, C, D, E, F, G, H, I)>();
    fn ehash(&self) -> u64 {
        let mut checksum = fletcher::Fletcher64::new();
        checksum.update(&split_u64(self.0.ehash()));
        checksum.update(&split_u64(self.1.ehash()));
        checksum.update(&split_u64(self.2.ehash()));
        checksum.update(&split_u64(self.3.ehash()));
        checksum.update(&split_u64(self.4.ehash()));

        checksum.update(&split_u64(self.4.ehash()));
        checksum.update(&split_u64(self.5.ehash()));
        checksum.update(&split_u64(self.6.ehash()));
        checksum.update(&split_u64(self.7.ehash()));
        checksum.update(&split_u64(self.8.ehash()));
        checksum.value()
    }
}

impl<A, B, C, D, E, F, G, H, I, J> EasyHash for (A, B, C, D, E, F, G, H, I, J)
where
    A: EasyHash,
    B: EasyHash,
    C: EasyHash,
    D: EasyHash,

    E: EasyHash,
    F: EasyHash,
    G: EasyHash,
    H: EasyHash,

    I: EasyHash,
    J: EasyHash,
{
    const TYPE_SALT: u32 = type_salt::<(A, B, C, D, E, F, G, H, I, J)>();
    fn ehash(&self) -> u64 {
        let mut checksum = fletcher::Fletcher64::new();
        checksum.update(&split_u64(self.0.ehash()));
        checksum.update(&split_u64(self.1.ehash()));
        checksum.update(&split_u64(self.2.ehash()));
        checksum.update(&split_u64(self.3.ehash()));
        checksum.update(&split_u64(self.4.ehash()));

        checksum.update(&split_u64(self.4.ehash()));
        checksum.update(&split_u64(self.5.ehash()));
        checksum.update(&split_u64(self.6.ehash()));
        checksum.update(&split_u64(self.7.ehash()));
        checksum.update(&split_u64(self.8.ehash()));
        checksum.update(&split_u64(self.9.ehash()));
        checksum.value()
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K> EasyHash for (A, B, C, D, E, F, G, H, I, J, K)
where
    A: EasyHash,
    B: EasyHash,
    C: EasyHash,
    D: EasyHash,

    E: EasyHash,
    F: EasyHash,
    G: EasyHash,
    H: EasyHash,

    I: EasyHash,
    J: EasyHash,
    K: EasyHash,
{
    const TYPE_SALT: u32 = type_salt::<(A, B, C, D, E, F, G, H, I, J, K)>();
    fn ehash(&self) -> u64 {
        let mut checksum = fletcher::Fletcher64::new();
        checksum.update(&split_u64(self.0.ehash()));
        checksum.update(&split_u64(self.1.ehash()));
        checksum.update(&split_u64(self.2.ehash()));
        checksum.update(&split_u64(self.3.ehash()));
        checksum.update(&split_u64(self.4.ehash()));

        checksum.update(&split_u64(self.4.ehash()));
        checksum.update(&split_u64(self.5.ehash()));
        checksum.update(&split_u64(self.6.ehash()));
        checksum.update(&split_u64(self.7.ehash()));
        checksum.update(&split_u64(self.8.ehash()));
        checksum.update(&split_u64(self.9.ehash()));
        checksum.update(&split_u64(self.10.ehash()));
        checksum.value()
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L> EasyHash for (A, B, C, D, E, F, G, H, I, J, K, L)
where
    A: EasyHash,
    B: EasyHash,
    C: EasyHash,
    D: EasyHash,

    E: EasyHash,
    F: EasyHash,
    G: EasyHash,
    H: EasyHash,

    I: EasyHash,
    J: EasyHash,
    K: EasyHash,
    L: EasyHash,
{
    const TYPE_SALT: u32 = type_salt::<(A, B, C, D, E, F, G, H, I, J, K, L)>();
    fn ehash(&self) -> u64 {
        let mut checksum = fletcher::Fletcher64::new();
        checksum.update(&split_u64(self.0.ehash()));
        checksum.update(&split_u64(self.1.ehash()));
        checksum.update(&split_u64(self.2.ehash()));
        checksum.update(&split_u64(self.3.ehash()));
        checksum.update(&split_u64(self.4.ehash()));

        checksum.update(&split_u64(self.4.ehash()));
        checksum.update(&split_u64(self.5.ehash()));
        checksum.update(&split_u64(self.6.ehash()));
        checksum.update(&split_u64(self.7.ehash()));
        checksum.update(&split_u64(self.8.ehash()));
        checksum.update(&split_u64(self.9.ehash()));
        checksum.update(&split_u64(self.10.ehash()));
        checksum.update(&split_u64(self.11.ehash()));
        checksum.value()
    }
}

// macro_rules! tuple_impls {
//     ( $head:ident, $( $tail:ident, )* ) => {
//         impl<$head, $( $tail ),*> EasyHash for ($head, $( $tail ),*)
//         where
//             $head: EasyHash,
//             $( $tail: EasyHash ),*
//         {
//             fn ehash(&self) -> u64 {

//                 // let mut s = DefaultHasher::new();
//                 // std::any::type_name::<bool>().hash(&mut s); //salt
//                 // self.hash(&mut s);
//                 // std::hash::Hasher::finish(&s)
//             }
//         }

//         tuple_impls!($( $tail, )*);
//     };

//     () => {
//         impl EasyHash for () {
//             fn ehash(&self) -> u64 {
//                 9827526396273 //random salt
//             }
//         }
//     };
// }

// tuple_impls!(A, B, C, D, E, F, G, H, I, J,);

// impl HeapSize for String {
//     /// A `String` owns enough heap memory to hold its reserved capacity.
//     fn heap_size_of_children(&self) -> usize {
//         self.capacity()
//     }
// }

// impl<T> HeapSize for Box<T>
// where
//     T: ?Sized + HeapSize,
// {
//     /// A `Box` owns however much heap memory was allocated to hold the value of
//     /// type `T` that we placed on the heap, plus transitively however much `T`
//     /// itself owns.
//     fn heap_size_of_children(&self) -> usize {
//         mem::size_of_val(&**self) + (**self).heap_size_of_children()
//     }
// }

// impl<T> HeapSize for [T]
// where
//     T: HeapSize,
// {
//     /// Sum of heap memory owned by each element of a dynamically sized slice of
//     /// `T`.
//     fn heap_size_of_children(&self) -> usize {
//         self.iter().map(HeapSize::heap_size_of_children).sum()
//     }
// }

// impl<'a, T> HeapSize for &'a T
// where
//     T: ?Sized,
// {
// //     fn heap_size_of_children(&self) -> usize {
//         0
//     }
// }
