macro_rules! forward_from_ref_binop {
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

macro_rules! forward_into_ref_binop {
    (impl $imp:ident, $method:ident for $t:ty where $($args:tt)*) => {
        impl<'a, $($args)*> $imp<$t> for &'a $t {
            type Output = $t;

            #[inline]
            fn $method(self, other: $t) -> $t {
                $imp::$method(*self, other)
            }
        }

        impl<'a, $($args)*> $imp<&'a $t> for $t {
            type Output = $t;

            #[inline]
            fn $method(self, other: &'a $t) -> $t {
                $imp::$method(self, *other)
            }
        }

        impl<'a, 'b, $($args)*> $imp<&'a $t> for &'b $t {
            type Output = $t;

            #[inline]
            fn $method(self, other: &'a $t) -> $t {
                $imp::$method(*self, *other)
            }
        }
    };
    (impl $imp:ident, $method:ident for $t:ty) => {
        impl<'a> $imp<$t> for &'a $t {
            type Output = $t;

            #[inline]
            fn $method(self, other: $t) -> $t {
                $imp::$method(*self, other)
            }
        }

        impl<'a> $imp<&'a $t> for $t {
            type Output = $t;

            #[inline]
            fn $method(self, other: &'a $t) -> $t {
                $imp::$method(self, *other)
            }
        }

        impl<'a, 'b> $imp<&'a $t> for &'b $t {
            type Output = $t;

            #[inline]
            fn $method(self, other: &'a $t) -> $t {
                $imp::$method(*self, *other)
            }
        }
    };
}

macro_rules! forward_from_ref_unop {
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

macro_rules! forward_into_ref_unop {
    (impl $imp:ident, $method:ident for $t:ty where $($args:tt)*) => {
        impl<'a, $($args)*> $imp for &'a $t {
            type Output = $t;

            #[inline]
            fn $method(self) -> $t {
                $imp::$method(*self)
            }
        }
    };
    (impl $imp:ident, $method:ident for $t:ty) => {
        impl<'a> $imp for &'a $t {
            type Output = $t;

            #[inline]
            fn $method(self) -> $t {
                $imp::$method(*self)
            }
        }
    };
}

macro_rules! forward_from_ref_assign {
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

macro_rules! forward_into_ref_assign {
    (impl $imp:ident, $method:ident for $t:ty where $($args:tt)*) => {
        impl<'a, $($args)*> $imp<&'a $t> for $t {
            #[inline]
            fn $method(&mut self, other: &'a $t) {
                $imp::$method(self, *other)
            }
        }
    };
    (impl $imp:ident, $method:ident for $t:ty) => {
        impl<'a> $imp<&'a $t> for $t {
            #[inline]
            fn $method(&mut self, other: &'a $t) {
                $imp::$method(self, *other)
            }
        }
    };
}

macro_rules! forward_from_ref_group {
    (impl Group for $t:ty) => {
        forward_from_ref_binop! { impl Add, add for $t }
        forward_from_ref_binop! { impl Sub, sub for $t }
        forward_from_ref_unop! { impl Neg, neg for $t }
        forward_from_ref_assign! { impl AddAssign, add_assign for $t }
        forward_from_ref_assign! { impl SubAssign, sub_assign for $t }
    };
    (impl Group for $t:ty where $($args:tt)*) => {
        forward_from_ref_binop! { impl Add, add for $t where $($args)* }
        forward_from_ref_binop! { impl Sub, sub for $t where $($args)* }
        forward_from_ref_unop! { impl Neg, neg for $t where $($args)*}
        forward_from_ref_assign! { impl AddAssign, add_assign for $t where $($args)* }
        forward_from_ref_assign! { impl SubAssign, sub_assign for $t where $($args)* }
    };
}

macro_rules! forward_into_ref_group {
    (impl Group for $t:ty) => {
        forward_into_ref_binop! { impl Add, add for $t }
        forward_into_ref_binop! { impl Sub, sub for $t }
        forward_into_ref_unop! { impl Neg, neg for $t }
        forward_into_ref_assign! { impl AddAssign, add_assign for $t }
        forward_into_ref_assign! { impl SubAssign, sub_assign for $t }
    };
    (impl Group for $t:ty where $($args:tt)*) => {
        forward_into_ref_binop! { impl Add, add for $t where $($args)* }
        forward_into_ref_binop! { impl Sub, sub for $t where $($args)* }
        forward_into_ref_unop! { impl Neg, neg for $t where $($args)*}
        forward_into_ref_assign! { impl AddAssign, add_assign for $t where $($args)* }
        forward_into_ref_assign! { impl SubAssign, sub_assign for $t where $($args)* }
    };
}

macro_rules! forward_from_ref_ring {
    (impl Ring for $t:ty) => {
        forward_from_ref_group! { impl Group for $t }
        forward_from_ref_binop! { impl Mul, mul for $t }
        forward_from_ref_assign! { impl MulAssign, mul_assign for $t }
    };
    (impl Ring for $t:ty where $($args:tt)*) => {
        forward_from_ref_group! { impl Group for $t where $($args)* }
        forward_from_ref_binop! { impl Mul, mul for $t where $($args)* }
        forward_from_ref_assign! { impl MulAssign, mul_assign for $t where $($args)* }
    };
}

macro_rules! forward_into_ref_ring {
    (impl Ring for $t:ty) => {
        forward_into_ref_group! { impl Group for $t }
        forward_into_ref_binop! { impl Mul, mul for $t }
        forward_into_ref_assign! { impl MulAssign, mul_assign for $t }
    };
    (impl Ring for $t:ty where $($args:tt)*) => {
        forward_into_ref_group! { impl Group for $t where $($args)* }
        forward_into_ref_binop! { impl Mul, mul for $t where $($args)* }
        forward_into_ref_assign! { impl MulAssign, mul_assign for $t where $($args)* }
    };
}

macro_rules! forward_from_ref_field {
    (impl Field for $t:ty) => {
        forward_from_ref_ring! { impl Ring for $t }
        forward_from_ref_binop! { impl Div, div for $t }
        forward_from_ref_assign! { impl DivAssign, div_assign for $t }
    };
    (impl Field for $t:ty where $($args:tt)*) => {
        forward_from_ref_ring! { impl Ring for $t where $($args)* }
        forward_from_ref_binop! { impl Div, div for $t where $($args)* }
        forward_from_ref_assign! { impl DivAssign, div_assign for $t where $($args)* }
    };
}

macro_rules! forward_into_ref_field {
    (impl Field for $t:ty) => {
        forward_into_ref_ring! { impl Ring for $t }
        forward_into_ref_binop! { impl Div, div for $t }
        forward_into_ref_assign! { impl DivAssign, div_assign for $t }
    };
    (impl Field for $t:ty where $($args:tt)*) => {
        forward_into_ref_ring! { impl Ring for $t where $($args)* }
        forward_into_ref_binop! { impl Div, div for $t where $($args)* }
        forward_into_ref_assign! { impl DivAssign, div_assign for $t where $($args)* }
    };
}

macro_rules! ring_from_str {
    (impl FromStr for $t:ty) => {
        impl std::str::FromStr for $t {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                if s.is_empty() {
                    return Err(());
                }
                let mut res = <$t>::from(0u32);
                let mut neg = false;
                for (i, c) in s.chars().enumerate() {
                    if i == 0 && c == '-' {
                        neg = true;
                        continue;
                    }
                    res *= <$t>::from(10u32);
                    match c.to_digit(10) {
                        Some(x) => res += <$t>::from(x),
                        None => return Err(())
                    }
                }
                if neg {
                    res = -res;
                }
                Ok(res)
            }
        }
    };
}

macro_rules! impl_zero_one_for_eq {
    (impl Zero, One for $t:ty, $z:expr, $u:expr) => {
        impl Zero for $t {
            fn zero() -> Self {
                $z
            }
            fn is_zero(&self) -> bool {
                *self == $z
            }
        }
        impl One for $t {
            fn one() -> Self {
                $u
            }
            fn is_one(&self) -> bool {
                *self == $u
            }
        }
    }
}

macro_rules! series_lookups {
    ( $n: ident, $s: ident, $( $x:ident ),* ) => {
        {
            match $n {
                $(
                    stringify!($x) => Some(Series::$x($s.default_precision)),
                )*
                _ => None
            }
        }
    };
}

macro_rules! function_lookups {
    ( $n: ident, $( $x:ident ),* ) => {
        {
            match $n {
                $(
                    stringify!($x) => Some(Series::$x),
                )*
                _ => None
            }
        }
    };
}

macro_rules! binop_promotion {
    ( $x: ident, $op: path, $y: ident) => {
        {
            match $x {
                SeriesExpr(s) =>  {
                    match $y {
                        SeriesExpr(t) => {
                            Ok(SeriesExpr($op(s, t)))
                        },
                        ConstExpr(d) => {
                            let mut ser = Series::promote(d);
                            ser.expand_to(s.accuracy());
                            Ok(SeriesExpr($op(s, ser)))
                        }
                    }
                },
                ConstExpr(c) => {
                    match $y {
                        SeriesExpr(t) => {
                            let mut ser = Series::promote(c);
                            ser.expand_to(t.accuracy());
                            Ok(SeriesExpr($op(ser, t)))
                        },
                        ConstExpr(d) => {
                            Ok(ConstExpr($op(c, d)))
                        }
                    }
                }
            }
        }
    };
}
