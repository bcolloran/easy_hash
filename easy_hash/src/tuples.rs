use crate::{EasyHash, join_u32s, type_salt, u64_to_u32_slice};

impl EasyHash for () {
    const TYPE_SALT: u32 = type_salt::<Self>();
    fn ehash(&self) -> u64 {
        join_u32s(Self::TYPE_SALT, Self::TYPE_SALT)
    }
}

macro_rules! easy_hash_tuple_impl {

    ( $($T:ident),+ ) => {
        #[allow(non_snake_case)]
        impl< $($T: EasyHash),+ > EasyHash for ( $($T,)+ ) {
            const TYPE_SALT: u32 = type_salt::<( $($T,)+ )>();

            fn ehash(&self) -> u64 {
                // 1) Make a new Fletcher64 checksum
                let mut checksum = fletcher::Fletcher64::new();
                // 2) Feed in the per‐type salt
                checksum.update(&[ Self::TYPE_SALT ]);

                // 3) Destructure self=(…) into bindings named after each type identifier:
                //
                //    let ( ref T0, ref T1, … ) = *self;
                //
                // Now `T0`, `T1`, … are local references to the tuple’s elements.
                let ( $( ref $T, )+ ) = *self;

                // 4) Build an array of each element’s `ehash()` and feed into the checksum.
                checksum.update(u64_to_u32_slice(&[ $( $T.ehash() ),+ ]));
                checksum.value()
            }
        }
    };
}

easy_hash_tuple_impl!(T0);
easy_hash_tuple_impl!(T0, T1);
easy_hash_tuple_impl!(T0, T1, T2);
easy_hash_tuple_impl!(T0, T1, T2, T3);
easy_hash_tuple_impl!(T0, T1, T2, T3, T4);
easy_hash_tuple_impl!(T0, T1, T2, T3, T4, T5);
easy_hash_tuple_impl!(T0, T1, T2, T3, T4, T5, T6);
easy_hash_tuple_impl!(T0, T1, T2, T3, T4, T5, T6, T7);
easy_hash_tuple_impl!(T0, T1, T2, T3, T4, T5, T6, T7, T8);
easy_hash_tuple_impl!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
easy_hash_tuple_impl!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
easy_hash_tuple_impl!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
easy_hash_tuple_impl!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
easy_hash_tuple_impl!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
easy_hash_tuple_impl!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14
);
