use crate::{PowerSeries, Field};
use std::ops::{Add, Sub, Mul, Neg, Div};

#[derive(Debug, Clone)]
enum PowerSeriesExpr<T: PowerSeries> {
    Leaf(T),
    Add(Box<PowerSeriesExpr<T>>, Box<PowerSeriesExpr<T>>),
    Sub(Box<PowerSeriesExpr<T>>, Box<PowerSeriesExpr<T>>),
    Mul(Box<PowerSeriesExpr<T>>, Box<PowerSeriesExpr<T>>),
    Div(Box<PowerSeriesExpr<T>>, Box<PowerSeriesExpr<T>>),
    Compose(Box<PowerSeriesExpr<T>>, Box<PowerSeriesExpr<T>>),
    Hadamard(Box<PowerSeriesExpr<T>>, Box<PowerSeriesExpr<T>>),
    Neg(Box<PowerSeriesExpr<T>>),
    LShift(Box<PowerSeriesExpr<T>>),
    RShift(Box<PowerSeriesExpr<T>>),
    Derive(Box<PowerSeriesExpr<T>>),
    Integrate(Box<PowerSeriesExpr<T>>),
    Inverse(Box<PowerSeriesExpr<T>>),
    Sqrt(Box<PowerSeriesExpr<T>>),
}

impl<T: PowerSeries> Default for PowerSeriesExpr<T> {
    fn default() -> Self {
        PowerSeriesExpr::Leaf(T::default())
    }
}

impl<T: PowerSeries> PartialEq for PowerSeriesExpr<T> {
    fn eq(&self, other: &Self) -> bool {
        self.eval() == other.eval()
    }
}

impl<T: PowerSeries> PowerSeriesExpr<T> {
    fn eval(&self) -> T {
        match self {
            PowerSeriesExpr::Leaf(x) => x.clone(),
            PowerSeriesExpr::Add(x, y) => x.eval() + y.eval(),
            PowerSeriesExpr::Sub(x, y) => x.eval() - y.eval(),
            PowerSeriesExpr::Mul(x, y) => x.eval() * y.eval(),
            PowerSeriesExpr::Div(x, y) => x.eval() / y.eval(),
            PowerSeriesExpr::Compose(x, y) => x.eval().compose(y.eval()),
            PowerSeriesExpr::Hadamard(x, y) => x.eval().hadamard(y.eval()),
            PowerSeriesExpr::Neg(x) => -x.eval(),
            PowerSeriesExpr::LShift(x) => x.eval().lshift(),
            PowerSeriesExpr::RShift(x) => x.eval().rshift(),
            PowerSeriesExpr::Derive(x) => x.eval().derive(),
            PowerSeriesExpr::Integrate(x) => x.eval().integrate(),
            PowerSeriesExpr::Inverse(x) => x.eval().inverse(),
            PowerSeriesExpr::Sqrt(x) => x.eval().sqrt(),
        }
    }
}

impl<T: PowerSeries> From<u32> for PowerSeriesExpr<T> {
    fn from(x: u32) -> Self {
        PowerSeriesExpr::Leaf(T::from(x))
    }
}

impl<T: PowerSeries> Add for PowerSeriesExpr<T> {
    type Output = Self;
    fn add(self, o: Self) -> Self {
        PowerSeriesExpr::Add(Box::new(self), Box::new(o))
    }
}

impl<T: PowerSeries> Sub for PowerSeriesExpr<T> {
    type Output = Self;
    fn sub(self, o: Self) -> Self {
        PowerSeriesExpr::Sub(Box::new(self), Box::new(o))
    }
}

impl<T: PowerSeries> Neg for PowerSeriesExpr<T> {
    type Output = Self;
    fn neg(self) -> Self {
        PowerSeriesExpr::Neg(Box::new(self))
    }
}

impl<T: PowerSeries> Mul for PowerSeriesExpr<T> {
    type Output = Self;
    fn mul(self, o: Self) -> Self {
        PowerSeriesExpr::Mul(Box::new(self), Box::new(o))
    }
}

impl<T: PowerSeries> Div for PowerSeriesExpr<T> {
    type Output = Self;
    fn div(self, o: Self) -> Self {
        PowerSeriesExpr::Div(Box::new(self), Box::new(o))
    }
}

impl<F: Field + Copy, T: PowerSeries<Coeff = F>> PowerSeries for PowerSeriesExpr<T> {
    type Coeff = F;
    fn promote(coeff: Self::Coeff) -> Self {
        PowerSeriesExpr::Leaf(T::promote(coeff))
    }
    fn identity() -> Self {
        PowerSeriesExpr::Leaf(T::identity())
    }
    fn coefficient(self, i: usize) -> Self::Coeff {
        self.eval().coefficient(i)
    }
    fn derive(self) -> Self {
        PowerSeriesExpr::Derive(Box::new(self))
    }
    fn integrate(self) -> Self {
        PowerSeriesExpr::Integrate(Box::new(self))
    }
    fn inverse(self) -> Self {
        PowerSeriesExpr::Inverse(Box::new(self))
    }
    fn compose(self, other: Self) -> Self {
        PowerSeriesExpr::Compose(Box::new(self), Box::new(other))
    }
    fn hadamard(self, other: Self) -> Self {
        PowerSeriesExpr::Hadamard(Box::new(self), Box::new(other))
    }
    fn sqrt(self) -> Self {
        PowerSeriesExpr::Sqrt(Box::new(self))
    }
    fn lshift(self) -> Self {
        PowerSeriesExpr::LShift(Box::new(self))
    }
    fn rshift(self) -> Self {
        PowerSeriesExpr::RShift(Box::new(self))
    }
}


/*
pub trait PowerSeriesExpr {
    type SeriesType: PowerSeries;
    fn eval(&self) -> Self::SeriesType; 
}

#[derive(Clone)]
pub struct LeafExpr<T: PowerSeries> {
    series: T
}

impl<T: PowerSeries> PowerSeriesExpr for LeafExpr<T> {
    type SeriesType = T;
    fn eval(&self) -> T { self.series }
}

type BoxedNode<T> = Box<dyn PowerSeriesExpr<SeriesType = T>>;

macro_rules! binop_type {
    ( $n:ident, $f:tt ) => {
        pub struct $n<T: PowerSeries> {
            left_expr: BoxedNode<T>,
            rght_expr: BoxedNode<T>
        }

        impl<T: PowerSeries> PowerSeriesExpr for $n<T> {
            type SeriesType = T;
            fn eval(&self) -> T { self.left_expr.eval().$f(self.rght_expr.eval()) }
        }
    };
}

binop_type!(AddExpr, add);
binop_type!(SubExpr, sub);
binop_type!(MulExpr, mul);
binop_type!(DivExpr, div);
binop_type!(ComposeExpr, compose);
binop_type!(HadamardExpr, hadamard);

macro_rules! unop_type {
    ( $n:ident, $f:tt ) => {
        pub struct $n<T: PowerSeries> {
            sub_expr: BoxedNode<T>
        }

        impl<T: PowerSeries> PowerSeriesExpr for $n<T> {
            type SeriesType = T;
            fn eval(&self) -> T { self.sub_expr.eval().$f() }
        }
    };
}

unop_type!(NegExpr, neg);
unop_type!(LShiftExpr, lshift);
unop_type!(RShiftExpr, rshift);
unop_type!(DeriveExpr, derive);
unop_type!(IntegrateExpr, integrate);
unop_type!(InverseExpr, inverse);
unop_type!(SqrtExpr, sqrt);

impl<T: PowerSeries> Add for BoxedNode<T> {
    type Output = BoxedNode<T>;
    fn add(self, o: BoxedNode<T>) -> BoxedNode<T> {
        Box::new(AddExpr {
            left_expr: self.clone(),
            rght_expr: o.clone()
        })
    }
}*/
