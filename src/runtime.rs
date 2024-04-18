use crate::{lexer, parser, Series};
use crate::lexer::Operator;
use crate::parser::SyntaxNode;
use rustc_hash::FxHashMap;
use crate::oeis::SeqDB;
use crate::powerseries::PowerSeries;
use crate::mathtypes::*;
use rug::Rational;
use std::str::FromStr;

pub struct RunTimeEnvironment {
    constant_variables: FxHashMap<String,Rational>,
    series_variables: FxHashMap<String,Series>,
    search_database: SeqDB,
    default_precision: usize,
}

#[derive(Debug)]
pub enum ExprValue {
    SeriesExpr(Series),
    ConstExpr(Rational)
}

impl ExprValue {
    fn apply_binop(x: ExprValue, op: Operator, y: ExprValue) -> Result<ExprValue, String> {
        match op {
            Operator::Add => {
                use std::ops::Add;
                binop_promotion!(x, Add::add, y)
            },
            Operator::Sub => {
                use std::ops::Sub;
                binop_promotion!(x, Sub::sub, y)
            },
            Operator::Mul => {
                use std::ops::Mul;
                binop_promotion!(x, Mul::mul, y)
            },
            Operator::Div => {
                match y {
                    SeriesExpr(ref s) => {
                        if s.is_zero() {
                            return Err("Division by zero.".to_string());
                        }
                    },
                    ConstExpr(ref c) => {
                        if c.is_zero() {
                            return Err("Division by zero.".to_string());
                        }
                    }
                }
                use std::ops::Div;
                binop_promotion!(x, Div::div, y)
            },
            Operator::Pow => {
                let ConstExpr(c) = y else {
                    return Err("Can not raise to power of series.".to_string());
                };
                let Ok(n) = i32::try_from(c.numer()) else {
                    return Err("Exponent does not fit in i32".to_string());
                };
                let Ok(m) = u32::try_from(c.denom()) else {
                    return Err("Exponent does not fit in u32".to_string());
                };
                match x {
                    ConstExpr(d) => {
                        if m != 1 {
                            return Err("Roots of rationals not possible".to_string())
                        }
                        use rug::ops::Pow;
                        Ok(ConstExpr(d.pow(n)))
                    },
                    SeriesExpr(s) => {
                        if m != 1 && !s[0].is_one() {
                            return Err("Roots of series with no constant not possible.".to_string());
                        }
                        Ok(SeriesExpr(s.ratpow(n, m)))
                    }
                }
            },
            Operator::Compose => {
                let SeriesExpr(xseq) = x else {
                    return Err("Can not compose with constant.".to_string());
                };
                let SeriesExpr(yseq) = y else {
                    return Err("Can not compose with constant.".to_string());
                };
                if !yseq[0].is_zero() {
                    return Err("Can not compose with series with non-zero constant.".to_string());
                }
                Ok(SeriesExpr(xseq.compose(&yseq)))
            },
            Operator::PointMul => {
                let SeriesExpr(s) = x else {
                    return Err("Can not do point multiplication on constant.".to_string());
                };
                let SeriesExpr(t) = y else {
                    return Err("Can not do point multiplication on constant.".to_string());
                };
                Ok(SeriesExpr(s.hadamard(&t)))
            }
            Operator::PointDiv => {
                let SeriesExpr(s) = x else {
                    return Err("Can not do point division on constant.".to_string());
                };
                let SeriesExpr(t) = y else {
                    return Err("Can not do point division on constant.".to_string());
                };
                for coeff in &t.seq {
                    if coeff.is_zero() {
                        return Err("Division by zero.".to_string())
                    }
                }
                Ok(SeriesExpr(s.point_div(&t)))
            }
            Operator::DefineEqual => {
                Err("Invalid use of define equals.".to_string())
            }
        }
    }
    fn apply_unop(op: Operator, x: ExprValue) -> Result<ExprValue, String> {
        match op {
            Operator::Sub => {
                Ok(match x {
                    SeriesExpr(s) => SeriesExpr(-s),
                    ConstExpr(s) => ConstExpr(-s)
                })
            },
            _ => Err("Unknown unary operator.".to_string())
        }
    }
}

impl std::fmt::Display for ExprValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SeriesExpr(s) => write!(f, "{}", s),
            ConstExpr(s) => write!(f, "{}", s)
        }
    }
}

use crate::runtime::ExprValue::*;

// TODO: If end with semicolon do not print
// TODO: Make default precision settable

impl RunTimeEnvironment {
    pub fn new(filename: String) -> std::io::Result<Self> {
        Ok(Self {
            constant_variables: Default::default(),
            series_variables: Default::default(),
            search_database: SeqDB::from_stripped(filename)?,
            default_precision: 16,
        })
    }

    fn transformation_lookup(name: &str) -> Option<fn(&Series) -> Series> {
        function_lookups!( name, point, sqrt, derive, integrate, log_derive, exp_integ, inverse, delta, partial_sums, partial_products, t019, laplace, laplace_inv, bous, bous_inv, mobius, mobius_inv, stirling, stirling_inv, euler, euler_inv, lah, lah_inv, powerset, lshift, rshift )
    }

    fn convolution_lookup(name: &str) -> Option<fn(&Series, &Series) -> Series> {
        function_lookups!( name, hadamard, exp_mul, dirichlet )
    }

    fn series_lookup(&self, name: &str) -> Option<Series> {
        series_lookups!( name, self, cos, sin, tan, expx, log1px )
    }

    fn evaluate(&mut self, node: &SyntaxNode) -> Result<ExprValue,String> {
        match node {
            SyntaxNode::Constant(s) => Ok(ExprValue::ConstExpr(
                    match Rational::from_str(
                        match std::str::from_utf8(s) {
                            Ok(s) => s,
                            Err(msg) => { return Err(msg.to_string()); }
                        }) {
                        Ok(r) => r,
                        Err(msg) => { return Err(msg.to_string()); }
                    }
            )),
            SyntaxNode::Identifier(s) => {
                let Ok(key) = std::str::from_utf8(s) else {
                    return Err("Invalid UTF8.".to_string());
                };
                if let Some(x) = self.series_variables.get(key) {
                    return Ok(SeriesExpr(x.clone()));
                }
                if let Some(x) = self.constant_variables.get(key) {
                    return Ok(ConstExpr(x.clone()));
                }
                if let Some(x) = self.series_lookup(key) {
                    return Ok(SeriesExpr(x));
                }
                if s.len() > 1 && s[0] == b'A' {
                    let mut all_dig = true;
                    for i in 1..s.len() {
                        all_dig &= b'0' <= s[i];
                        all_dig &= s[i] <= b'9';
                    }
                    if all_dig {
                        let Ok(a_str) = std::str::from_utf8(&s[1..]) else {
                            return Err("Invalid UTF8.".to_string());
                        };
                        let Ok(a_num) = a_str.parse() else {
                            return Err("Failed to parse A number".to_string());
                        };
                        let Some(ind) = self.search_database.a_to_ind.get(&a_num) else {
                            return Err("Did not find sequence with given A number in database.".to_string());
                        };
                        let mut ret = self.search_database.long_vec[*ind].clone();
                        ret.limit_accuracy(self.default_precision);
                        return Ok(SeriesExpr(ret));
                    }
                }
                Err("Identifier not found.".to_string())
            },
            SyntaxNode::Series(s) => {
                let mut coeff = vec![];
                for c in s {
                    let ConstExpr(res) = self.evaluate(c)? else {
                        return Err("Expected const in series definition.".to_string());
                    };
                    coeff.push(res);
                }
                Ok(SeriesExpr(Series { seq: coeff }))
            },
            SyntaxNode::Paren(s) => self.evaluate(s),
            SyntaxNode::LetStatement(name, expr) => {
                let Ok(key) = std::str::from_utf8(name) else {
                    return Err("Failed to parse variable name".to_string());
                };
                match self.evaluate(expr)? {
                    SeriesExpr(s) => { self.series_variables.insert(key.to_string(), s); },
                    ConstExpr(s) => { self.constant_variables.insert(key.to_string(), s); }
                }
                Ok(ConstExpr(Default::default()))
            }
            SyntaxNode::RecStatement(name, expr) => {
                Err("Rec has not been implemented yet!".to_string())
            },
            SyntaxNode::Application(func, args) => {
                let Ok(name) = std::str::from_utf8(func) else {
                    return Err("Failed to parse function name".to_string());
                };
                let mut arg_vals = vec![];
                for arg in args {
                    match self.evaluate(arg)? {
                        SeriesExpr(s) => arg_vals.push(s),
                        ConstExpr(c) => arg_vals.push(Series::promote(c))
                    }
                }
                match name {
                    "help" => { 
                        return Err("Help not implemented yet.".to_string());
                    },
                    "search" => {
                        if args.len() != 1 {
                            return Err("Search takes one argument.".to_string());
                        }
                        let topres = self.search_database.search_full(&arg_vals[0]);
                        return Err(topres.to_string());
                    },
                    "set_precision" => {
                        return Err("Set precision not implemented yet.".to_string());
                    },
                    _ => { }
                }
                match arg_vals.len() {
                    1 => {
                        let Some(f) = Self::transformation_lookup(name) else {
                            return Err("Function not found".to_string());
                        };
                        Ok(SeriesExpr(f(&arg_vals[0])))
                    },
                    2 => {
                        let Some(f) = Self::convolution_lookup(name) else {
                            return Err("Function not found".to_string());
                        };
                        Ok(SeriesExpr(f(&arg_vals[0], &arg_vals[1])))
                    }
                    _ => Err("Function not found".to_string())
                }
            },
            SyntaxNode::BinaryOp(x, op, y) => {
                Ok(ExprValue::apply_binop(self.evaluate(x)?, *op, self.evaluate(y)?)?)
            },
            SyntaxNode::UnaryOp(op, x) => {
                Ok(ExprValue::apply_unop(*op, self.evaluate(x)?)?)
            }
        }
    }

    pub fn repl(&mut self, inp: &mut dyn std::io::BufRead, outp: &mut dyn std::io::Write, interactive: bool) -> std::io::Result<()> {
        let mut buf: String = Default::default();
        loop {
            buf.clear();
            match inp.read_line(&mut buf) {
                Err(_) => { return Ok(()); },
                Ok(0) => { return Ok(()); }
                _ => { }
            }
            let tokens = match lexer::parse_tokens(buf.as_bytes()) {
                Ok(x) => { x },
                Err(msg) => {
                    outp.write(msg.as_bytes())?;
                    outp.write(b"\n")?;
                    if interactive { outp.flush()?; }
                    continue;
                }
            };
            let syntax_tree = match parser::parse_commands(&tokens) {
                Ok(x) => { x },
                Err(msg) => {
                    outp.write(msg.as_bytes())?;
                    outp.write(b"\n")?;
                    if interactive { outp.flush()?; }
                    continue;
                }
            };
            for node in syntax_tree {
                match self.evaluate(&node) {
                    Ok(x) => { 
                        outp.write(x.to_string().as_bytes())?;
                        outp.write(b"\n")?;
                        if interactive { outp.flush()?; }
                    },
                    Err(msg) => {
                        outp.write(msg.as_bytes())?;
                        outp.write(b"\n")?;
                        if interactive { outp.flush()?; }
                    }
                };
            }
        }
    }
}
