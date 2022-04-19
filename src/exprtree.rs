use crate::{PowerSeries, Field};
use std::ops::{Add, Sub, Mul, Neg, Div};

#[derive(Debug, Clone)]
enum PowerSeriesExpr<T: PowerSeries> {
    LeafExpr(T),
    AddExpr(Box<PowerSeriesExpr<T>>, Box<PowerSeriesExpr<T>>),
    SubExpr(Box<PowerSeriesExpr<T>>, Box<PowerSeriesExpr<T>>),
    MulExpr(Box<PowerSeriesExpr<T>>, Box<PowerSeriesExpr<T>>),
    DivExpr(Box<PowerSeriesExpr<T>>, Box<PowerSeriesExpr<T>>),
    ComposeExpr(Box<PowerSeriesExpr<T>>, Box<PowerSeriesExpr<T>>),
    HadamardExpr(Box<PowerSeriesExpr<T>>, Box<PowerSeriesExpr<T>>),
    NegExpr(Box<PowerSeriesExpr<T>>),
    LShiftExpr(Box<PowerSeriesExpr<T>>),
    RShiftExpr(Box<PowerSeriesExpr<T>>),
    DeriveExpr(Box<PowerSeriesExpr<T>>),
    IntegrateExpr(Box<PowerSeriesExpr<T>>),
    InverseExpr(Box<PowerSeriesExpr<T>>),
    SqrtExpr(Box<PowerSeriesExpr<T>>),
}

impl<T: PowerSeries> Default for PowerSeriesExpr<T> {
    fn default() -> Self {
        PowerSeriesExpr::LeafExpr(T::default())
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
            PowerSeriesExpr::LeafExpr(x) => x.clone(),
            PowerSeriesExpr::AddExpr(x, y) => x.eval() + y.eval(),
            PowerSeriesExpr::SubExpr(x, y) => x.eval() - y.eval(),
            PowerSeriesExpr::MulExpr(x, y) => x.eval() * y.eval(),
            PowerSeriesExpr::DivExpr(x, y) => x.eval() / y.eval(),
            PowerSeriesExpr::ComposeExpr(x, y) => x.eval().compose(y.eval()),
            PowerSeriesExpr::HadamardExpr(x, y) => x.eval().hadamard(y.eval()),
            PowerSeriesExpr::NegExpr(x) => -x.eval(),
            PowerSeriesExpr::LShiftExpr(x) => x.eval().lshift(),
            PowerSeriesExpr::RShiftExpr(x) => x.eval().rshift(),
            PowerSeriesExpr::DeriveExpr(x) => x.eval().derive(),
            PowerSeriesExpr::IntegrateExpr(x) => x.eval().integrate(),
            PowerSeriesExpr::InverseExpr(x) => x.eval().inverse(),
            PowerSeriesExpr::SqrtExpr(x) => x.eval().sqrt(),
        }
    }
}

impl<T: PowerSeries> From<u32> for PowerSeriesExpr<T> {
    fn from(x: u32) -> Self {
        PowerSeriesExpr::LeafExpr(T::from(x))
    }
}

impl<T: PowerSeries> Add for PowerSeriesExpr<T> {
    type Output = Self;
    fn add(self, o: Self) -> Self {
        PowerSeriesExpr::AddExpr(Box::new(self), Box::new(o))
    }
}

impl<T: PowerSeries> Sub for PowerSeriesExpr<T> {
    type Output = Self;
    fn sub(self, o: Self) -> Self {
        PowerSeriesExpr::SubExpr(Box::new(self), Box::new(o))
    }
}

impl<T: PowerSeries> Neg for PowerSeriesExpr<T> {
    type Output = Self;
    fn neg(self) -> Self {
        PowerSeriesExpr::NegExpr(Box::new(self))
    }
}

impl<T: PowerSeries> Mul for PowerSeriesExpr<T> {
    type Output = Self;
    fn mul(self, o: Self) -> Self {
        PowerSeriesExpr::MulExpr(Box::new(self), Box::new(o))
    }
}

impl<T: PowerSeries> Div for PowerSeriesExpr<T> {
    type Output = Self;
    fn div(self, o: Self) -> Self {
        PowerSeriesExpr::DivExpr(Box::new(self), Box::new(o))
    }
}

impl<F: Field + Copy, T: PowerSeries<Coeff = F>> PowerSeries for PowerSeriesExpr<T> {
    type Coeff = F;
    fn promote(coeff: Self::Coeff) -> Self {
        PowerSeriesExpr::LeafExpr(T::promote(coeff))
    }
    fn identity() -> Self {
        PowerSeriesExpr::LeafExpr(T::identity())
    }
    fn coefficient(self, i: usize) -> Self::Coeff {
        self.eval().coefficient(i)
    }
    fn derive(self) -> Self {
        PowerSeriesExpr::DeriveExpr(Box::new(self))
    }
    fn integrate(self) -> Self {
        PowerSeriesExpr::IntegrateExpr(Box::new(self))
    }
    fn inverse(self) -> Self {
        PowerSeriesExpr::InverseExpr(Box::new(self))
    }
    fn compose(self, other: Self) -> Self {
        PowerSeriesExpr::ComposeExpr(Box::new(self), Box::new(other))
    }
    fn hadamard(self, other: Self) -> Self {
        PowerSeriesExpr::HadamardExpr(Box::new(self), Box::new(other))
    }
    fn sqrt(self) -> Self {
        PowerSeriesExpr::SqrtExpr(Box::new(self))
    }
    fn lshift(self) -> Self {
        PowerSeriesExpr::LShiftExpr(Box::new(self))
    }
    fn rshift(self) -> Self {
        PowerSeriesExpr::RShiftExpr(Box::new(self))
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