macro_rules! forward_binop_impl {
    (impl $imp:ident, $method:ident for $t:ty where $($args:tt)*) => {
        impl<'a, $($args)*> $imp<$t> for &'a $t {
            type Output = $t;

            #[inline]
            fn $method(self, other: $t) -> $t {
                $imp::$method(self, &other)
            }
        }

        impl<'a, $($args)*> $imp<&'a $t> for $t {
            type Output = $t;

            #[inline]
            fn $method(self, other: &'a $t) -> $t {
                $imp::$method(&self, other)
            }
        }

        impl<$($args)*> $imp<$t> for $t {
            type Output = $t;

            #[inline]
            fn $method(self, other: $t) -> $t {
                $imp::$method(&self, &other)
            }
        }
    };
    (impl $imp:ident, $method:ident for $t:ty) => {
        impl<'a> $imp<$t> for &'a $t {
            type Output = $t;

            #[inline]
            fn $method(self, other: $t) -> $t {
                $imp::$method(self, &other)
            }
        }

        impl<'a> $imp<&'a $t> for $t {
            type Output = $t;

            #[inline]
            fn $method(self, other: &'a $t) -> $t {
                $imp::$method(&self, other)
            }
        }

        impl $imp<$t> for $t {
            type Output = $t;

            #[inline]
            fn $method(self, other: $t) -> $t {
                $imp::$method(&self, &other)
            }
        }
    };
}

macro_rules! forward_unop_impl {
    (impl $imp:ident, $method:ident for $t:ty where $($args:tt)*) => {
        impl<$($args)*> $imp for $t {
            type Output = $t;

            #[inline]
            fn $method(self) -> $t {
                $imp::$method(&self)
            }
        }
    };
    (impl $imp:ident, $method:ident for $t:ty) => {
        impl $imp for $t {
            type Output = $t;

            #[inline]
            fn $method(self) -> $t {
                $imp::$method(&self)
            }
        }
    };
}

macro_rules! forward_assign_impl {
    (impl $imp:ident, $method:ident for $t:ty where $($args:tt)*) => {
        impl<$($args)*> $imp for $t {
            #[inline]
            fn $method(&mut self, other: $t) {
                $imp::$method(self, &other)
            }
        }
    };
    (impl $imp:ident, $method:ident for $t:ty) => {
        impl $imp for $t {
            #[inline]
            fn $method(&mut self, other: $t) {
                $imp::$method(self, &other)
            }
        }
    };
}
