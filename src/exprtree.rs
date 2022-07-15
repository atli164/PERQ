use crate::{PowerSeries, Field};
use std::ops::{Add, Sub, Mul, Neg, Div};

#[derive(Debug, Clone)]
enum SeqExprTree<T: PowerSeries> {
    Leaf(T),
    Add(Box<SeqExprTree<T>>, Box<SeqExprTree<T>>),
    Sub(Box<SeqExprTree<T>>, Box<SeqExprTree<T>>),
    Mul(Box<SeqExprTree<T>>, Box<SeqExprTree<T>>),
    Div(Box<SeqExprTree<T>>, Box<SeqExprTree<T>>),
    Compose(Box<SeqExprTree<T>>, Box<SeqExprTree<T>>),
    Hadamard(Box<SeqExprTree<T>>, Box<SeqExprTree<T>>),
    Neg(Box<SeqExprTree<T>>),
    LShift(Box<SeqExprTree<T>>),
    RShift(Box<SeqExprTree<T>>),
    Derive(Box<SeqExprTree<T>>),
    Integrate(Box<SeqExprTree<T>>),
    Inverse(Box<SeqExprTree<T>>),
    Sqrt(Box<SeqExprTree<T>>),
}

impl<T: PowerSeries> Default for SeqExprTree<T> {
    fn default() -> Self {
        SeqExprTree::Leaf(T::default())
    }
}

impl<T: PowerSeries> PartialEq for SeqExprTree<T> {
    fn eq(&self, other: &Self) -> bool {
        self.eval() == other.eval()
    }
}

impl<T: PowerSeries> SeqExprTree<T> {
    fn eval(&self) -> T {
        match self {
            SeqExprTree::Leaf(x) => x.clone(),
            SeqExprTree::Add(x, y) => x.eval() + y.eval(),
            SeqExprTree::Sub(x, y) => x.eval() - y.eval(),
            SeqExprTree::Mul(x, y) => x.eval() * y.eval(),
            SeqExprTree::Div(x, y) => x.eval() / y.eval(),
            SeqExprTree::Compose(x, y) => x.eval().compose(&y.eval()),
            SeqExprTree::Hadamard(x, y) => x.eval().hadamard(&y.eval()),
            SeqExprTree::Neg(x) => -x.eval(),
            SeqExprTree::LShift(x) => x.eval().lshift(),
            SeqExprTree::RShift(x) => x.eval().rshift(),
            SeqExprTree::Derive(x) => x.eval().derive(),
            SeqExprTree::Integrate(x) => x.eval().integrate(),
            SeqExprTree::Inverse(x) => x.eval().inverse(),
            SeqExprTree::Sqrt(x) => x.eval().sqrt(),
        }
    }
}

impl<T: PowerSeries> From<u32> for SeqExprTree<T> {
    fn from(x: u32) -> Self {
        SeqExprTree::Leaf(T::from(x))
    }
}

impl<T: PowerSeries> Add for SeqExprTree<T> {
    type Output = Self;
    fn add(self, o: Self) -> Self {
        SeqExprTree::Add(Box::new(self), Box::new(o))
    }
}

impl<T: PowerSeries> Sub for SeqExprTree<T> {
    type Output = Self;
    fn sub(self, o: Self) -> Self {
        SeqExprTree::Sub(Box::new(self), Box::new(o))
    }
}

impl<T: PowerSeries> Neg for SeqExprTree<T> {
    type Output = Self;
    fn neg(self) -> Self {
        SeqExprTree::Neg(Box::new(self))
    }
}

impl<T: PowerSeries> Mul for SeqExprTree<T> {
    type Output = Self;
    fn mul(self, o: Self) -> Self {
        SeqExprTree::Mul(Box::new(self), Box::new(o))
    }
}

impl<T: PowerSeries> Div for SeqExprTree<T> {
    type Output = Self;
    fn div(self, o: Self) -> Self {
        SeqExprTree::Div(Box::new(self), Box::new(o))
    }
}

impl<F: Field + Copy, T: PowerSeries<Coeff = F>> PowerSeries for SeqExprTree<T> {
    type Coeff = F;
    fn promote(coeff: Self::Coeff) -> Self {
        SeqExprTree::Leaf(T::promote(coeff))
    }
    fn identity() -> Self {
        SeqExprTree::Leaf(T::identity())
    }
    fn coefficient(self, i: usize) -> Self::Coeff {
        self.eval().coefficient(i)
    }
    fn derive(&self) -> Self {
        SeqExprTree::Derive(Box::new(self))
    }
    fn integrate(&self) -> Self {
        SeqExprTree::Integrate(Box::new(self))
    }
    fn inverse(&self) -> Self {
        SeqExprTree::Inverse(Box::new(self))
    }
    fn compose(&self, other: Self) -> Self {
        SeqExprTree::Compose(Box::new(self), Box::new(other))
    }
    fn hadamard(&self, other: Self) -> Self {
        SeqExprTree::Hadamard(Box::new(self), Box::new(other))
    }
    fn sqrt(&self) -> Self {
        SeqExprTree::Sqrt(Box::new(self))
    }
    fn lshift(&self) -> Self {
        SeqExprTree::LShift(Box::new(self))
    }
    fn rshift(&self) -> Self {
        SeqExprTree::RShift(Box::new(self))
    }
}


/*
pub trait SeqExprTree {
    type SeriesType: PowerSeries;
    fn eval(&self) -> Self::SeriesType; 
}

#[derive(Clone)]
pub struct LeafExpr<T: PowerSeries> {
    series: T
}

impl<T: PowerSeries> SeqExprTree for LeafExpr<T> {
    type SeriesType = T;
    fn eval(&self) -> T { self.series }
}

type BoxedNode<T> = Box<dyn SeqExprTree<SeriesType = T>>;

macro_rules! binop_type {
    ( $n:ident, $f:tt ) => {
        pub struct $n<T: PowerSeries> {
            left_expr: BoxedNode<T>,
            rght_expr: BoxedNode<T>
        }

        impl<T: PowerSeries> SeqExprTree for $n<T> {
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

        impl<T: PowerSeries> SeqExprTree for $n<T> {
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
