/// Extend a unary operator trait impl over refs.
///
/// Given an implementation of `op T` where T is `Copy`able, implements the unary
/// operator `op &T`.
#[macro_export]
macro_rules! forward_ref_unop {
    (impl$(<$($T:ident $(: $b0:ident $(+$b:ident)*)?),*>)? $imp:ident, $method:ident for $t:ty) => {
        impl$(<$($T $(: $b0 $(+$b)*)?),*>)? $imp for &$t {
            type Output = <$t as $imp>::Output;

            #[inline]
            fn $method(self) -> <$t as $imp>::Output {
                $imp::$method(*self)
            }
        }
    };
}

/// Extend a binary operator trait impl over refs.
///
/// Given an implementation of `T op U` where T and U are `Copy`able, implements
/// the binary operators:
/// - `&T op U`
/// - `T op &U`
/// - `&T op &U`
#[macro_export]
macro_rules! forward_ref_binop {
    (impl$(<$($T:ident $(: $b0:ident $(+$b:ident)*)?),*>)? $imp:ident, $method:ident for $t:ty, $u:ty) => {
        impl<'a, $($($T $(: $b0 $(+$b)*)?),*)?> $imp<$u> for &'a $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, other)
            }
        }

        impl$(<$($T $(: $b0 $(+$b)*)?),*>)? $imp<&$u> for $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: &$u) -> <$t as $imp<$u>>::Output {
                $imp::$method(self, *other)
            }
        }

        impl$(<$($T $(: $b0 $(+$b)*)?),*>)? $imp<&$u> for &$t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: &$u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, *other)
            }
        }
    };
}

/// Extend an assignment operator trait impl over refs.
///
/// Given an implementation of `T op= U` where U is `Copy`able, implements
/// the binary operator `T op= &U`.
#[macro_export]
macro_rules! forward_ref_op_assign {
    (impl$(<$($T:ident $(: $b0:ident $(+$b:ident)*)?),*>)? $imp:ident, $method:ident for $t:ty, $u:ty) => {
        impl$(<$($T $(: $b0 $(+$b)*)?),*>)? $imp<&$u> for $t {
            #[inline]
            fn $method(&mut self, other: &$u) {
                $imp::$method(self, *other);
            }
        }
    };
}
