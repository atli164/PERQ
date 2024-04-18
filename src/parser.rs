use crate::lexer::{Token, Operator};

// TODO: Fix unary operator fuckery (due to split)
// TODO: Fix right vs left associative (right chunk)

#[derive(Debug, Clone, PartialEq)]
pub enum SyntaxNode {
    Constant(Vec<u8>),
    Identifier(Vec<u8>),
    Series(Vec<SyntaxNode>),
    Paren(Box<SyntaxNode>),
    LetStatement(Vec<u8>, Box<SyntaxNode>),
    RecStatement(Box<SyntaxNode>, Box<SyntaxNode>),
    Application(Vec<u8>, Vec<SyntaxNode>),
    BinaryOp(Box<SyntaxNode>, Operator, Box<SyntaxNode>),
    UnaryOp(Operator, Box<SyntaxNode>),
}

type ParserFunction = fn(&[Token]) -> Result<SyntaxNode, String>;

fn check_unary_op(tok: &[Token], op: Operator, subroutine: ParserFunction) -> Result<Option<SyntaxNode>, String> {
    Ok(if tok[0] == Token::Operator(op) {
        Some(subroutine(&tok[1..])?)
    } else {
        None
    })
}

fn right_binary_chunk<const N: usize>(tok: &[Token], ops: [Operator; N], subroutine: ParserFunction) -> Result<SyntaxNode, String> {
    let mut depth = 0;
    let mut op_loc = vec![];
    for i in 0..tok.len() {
        if tok[i] == Token::OpenParen {
            depth += 1;
        }
        if tok[i] == Token::OpenBracket {
            depth += 1;
        }
        if depth == 0 {
            for j in 0..N {
                if tok[i] == Token::Operator(ops[j]) {
                    op_loc.push((i, j));
                }
            }
        }
        if tok[i] == Token::CloseParen {
            depth -= 1;
        }
        if tok[i] == Token::CloseBracket {
            depth -= 1;
        }
    }
    if op_loc.len() == 0 {
        return subroutine(tok);
    }
    let mut cur_node = subroutine(&tok[op_loc[op_loc.len() - 1].0+1..])?;
    while op_loc.len() > 1 {
        let lhs = subroutine(&tok[op_loc[op_loc.len() - 2].0+1..op_loc[op_loc.len() - 1].0])?;
        cur_node = SyntaxNode::BinaryOp(Box::new(lhs), ops[op_loc[op_loc.len() - 1].1], Box::new(cur_node));
        op_loc.pop();
    }
    if op_loc.len() == 1 {
        let lhs = subroutine(&tok[..op_loc[0].0])?;
        cur_node = SyntaxNode::BinaryOp(Box::new(lhs), ops[op_loc[0].1], Box::new(cur_node));
    }
    Ok(cur_node)
}

fn split_on_delim(tok: &[Token], delim: Token, subroutine: ParserFunction) -> Result<Vec<SyntaxNode>, String> {
    let mut res = vec![];
    let mut i = 0;
    let mut depth = 0;
    while i < tok.len() {
        let mut j = i;
        while j < tok.len() && (tok[j] != delim || depth > 0) { 
            if tok[j] == Token::OpenParen || tok[j] == Token::OpenBracket { depth += 1; }
            if tok[j] == Token::CloseParen || tok[j] == Token::CloseBracket { depth -= 1; }
            j += 1; 
        }
        res.push(subroutine(&tok[i..j])?);
        i = j + 1;
    }
    Ok(res)
}

fn parse_constant_literal(tok: &[Token]) -> Result<SyntaxNode, String> {
    if tok.len() != 1 {
        return Err("Expected single constant literal".to_string());
    }
    match &tok[0] {
        Token::Literal(x) => Ok(SyntaxNode::Constant(x.to_vec())),
        _ => Err("Expected literal".to_string())
    }
}

fn parse_constant_operand(tok: &[Token]) -> Result<SyntaxNode, String> {
    match check_unary_op(tok, Operator::Sub, parse_constant_operand)? {
        Some(res) => Ok(SyntaxNode::UnaryOp(Operator::Sub, Box::new(res))),
        None => right_binary_chunk(tok, [Operator::Pow], parse_constant_literal)
    }
}

fn parse_constant_factor(tok: &[Token]) -> Result<SyntaxNode, String> {
    right_binary_chunk(tok, [Operator::Mul, Operator::Div], parse_constant_operand)
}

fn parse_constant_summand(tok: &[Token]) -> Result<SyntaxNode, String> {
    right_binary_chunk(tok, [Operator::Add, Operator::Sub], parse_constant_factor)
}

fn parse_expression(tok: &[Token]) -> Result<SyntaxNode, String> {
    // options:
    // num
    // [num, num, num, ...]
    // identifier
    // identifier "(" expr , expr , expr ... ")"
    // identifier expr
    // "(" summand ")"
    match &tok[0] {
        Token::Identifier(x) => {
            if tok.len() == 1 {
                Ok(SyntaxNode::Identifier(x.to_vec()))
            } else if tok[1] == Token::OpenParen {
                let args = split_on_delim(&tok[2..tok.len()-1], Token::Comma, parse_summand)?;
                Ok(SyntaxNode::Application(x.to_vec(), args))
            } else {
                let arg = parse_expression(&tok[1..])?;
                Ok(SyntaxNode::Application(x.to_vec(), vec![arg]))
            }
        },
        Token::OpenParen => {
            parse_summand(&tok[1..tok.len()-1])
        },
        Token::OpenBracket => {
            let coeffs = split_on_delim(&tok[1..tok.len()-1], Token::Comma, parse_constant_summand)?;
            Ok(SyntaxNode::Series(coeffs))
        },
        _ => {
            parse_constant_summand(tok)
        }
    }
}

fn parse_operand(tok: &[Token]) -> Result<SyntaxNode, String> {
    match check_unary_op(tok, Operator::Sub, parse_operand)? {
        Some(res) => Ok(SyntaxNode::UnaryOp(Operator::Sub, Box::new(res))),
        None => right_binary_chunk(tok, [Operator::Pow, Operator::Compose], parse_expression)
    }
}

fn parse_factor(tok: &[Token]) -> Result<SyntaxNode, String> {
    right_binary_chunk(tok, [Operator::Mul, Operator::Div, Operator::PointMul, Operator::PointDiv], parse_operand)
}

fn parse_summand(tok: &[Token]) -> Result<SyntaxNode, String> {
    right_binary_chunk(tok, [Operator::Add, Operator::Sub], parse_factor)
}

fn parse_command(tok: &[Token]) -> Result<SyntaxNode, String> {
    if tok.len() == 0 {
        return Err("Expected token, got nothing".to_string());
    }
    Ok(match tok[0] {
        Token::Let => {
            if tok.len() <= 3 {
                return Err("Expected identifier := expression to follow let keyword".to_string());
            }
            let lhs = match &tok[1] {
                Token::Identifier(x) => x,
                _ => { return Err("Expected identifier expression to follow let keyword".to_string()); }
            };
            if tok[2] != Token::Operator(Operator::DefineEqual) {
                return Err("Expected := to follow let keyword".to_string());
            }
            let rhs = parse_summand(&tok[3..])?;
            SyntaxNode::LetStatement(lhs.to_vec(), Box::new(rhs))
        },
        Token::LetRec => {
            let define_loc = match tok.iter().position(|x| x == &Token::Operator(Operator::DefineEqual)) {
                Some(x) => x,
                None => { return Err("Expected := to follow letrec keyword".to_string()); }
            };
            let lhs = parse_summand(&tok[1..define_loc])?;
            let rhs = parse_summand(&tok[define_loc+1..])?;
            SyntaxNode::RecStatement(Box::new(lhs), Box::new(rhs))
        }
        _ => { 
            parse_summand(tok)?
        }
    })
}

pub fn parse_commands(tok: &[Token]) -> Result<Vec<SyntaxNode>, String> {
    let mut command_nodes = vec![];
    let mut i = 0; 
    while i < tok.len() {
        let mut j = i;
        while j < tok.len() && tok[j] != Token::Semicolon { j += 1; }
        command_nodes.push(parse_command(&tok[i..j])?);
        i = j;
        if i < tok.len() && tok[i] == Token::Semicolon { i += 1; }
    }
    Ok(command_nodes)
}

#[test]
fn parser_test_1() {
    use crate::parser::SyntaxNode::*;
    let tok = crate::lexer::parse_tokens("let x := [7, 2, 3]".as_bytes()).unwrap();
    let res = crate::parser::parse_commands(&tok).unwrap();
    let expected = [
        LetStatement(vec![b'x'], 
            Box::new(Series(vec![
                Constant(vec![b'7']), 
                Constant(vec![b'2']), 
                Constant(vec![b'3'])
            ]))
        )
    ];
    assert!(res == expected);
}
