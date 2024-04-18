#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Compose,
    PointMul,
    PointDiv,
    DefineEqual,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Operator(Operator),
    Identifier(Vec<u8>),
    Literal(Vec<u8>),
    Comma,
    Semicolon,
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    Let,
    LetRec,
}

fn match_from_list<T, const N: usize>(text: &[u8], pos: &mut usize, list: [(&str, T); N]) -> Option<T> {
    for (name, res) in list {
        if *pos + name.len() <= text.len() && text[*pos..*pos+name.len()] == *name.as_bytes() {
            *pos += name.len();
            return Some(res);
        }
    }
    None
}

const OPERATORS: [(&str, Operator); 9] = [
    ("+", Operator::Add),
    ("-", Operator::Sub),
    ("*", Operator::Mul),
    ("/", Operator::Div),
    ("^", Operator::Pow),
    ("@", Operator::Compose),
    (".*", Operator::PointMul),
    ("./", Operator::PointDiv),
    (":=", Operator::DefineEqual),
];

const KEYWORDS: [(&str, Token); 2] = [
    ("let", Token::Let), 
    ("letrec", Token::LetRec),
];

pub fn parse_tokens(text: &[u8]) -> Result<Vec<Token>,String> {
    let mut res = vec![];
    let mut pos = 0;
    while pos < text.len() {
        if text[pos] != b'\n' && text[pos].is_ascii_whitespace() {
            pos += 1;
            continue;
        }
        res.push(match text[pos] {
            b',' => { pos += 1; Token::Comma },
            b';' | b'\n' => { pos += 1; Token::Semicolon },
            b'(' => { pos += 1; Token::OpenParen },
            b')' => { pos += 1; Token::CloseParen },
            b'[' => { pos += 1; Token::OpenBracket },
            b']' => { pos += 1; Token::CloseBracket },
            b'0' ..= b'9' => {
                let mut end = pos + 1;
                while end < text.len() && text[end].is_ascii_digit() { end += 1; }
                let num = text[pos..end].to_vec();
                pos = end;
                Token::Literal(num)
            },
            b'a' ..= b'z' | b'A' ..= b'Z' | b'_' => {
                match match_from_list(text, &mut pos, KEYWORDS) {
                    Some(token) => token,
                    None => {
                        let mut end = pos + 1;
                        while end < text.len() && (text[end].is_ascii_alphanumeric() || text[end] == b'_') { end += 1; }
                        let name = text[pos..end].to_vec();
                        pos = end;
                        Token::Identifier(name)
                    }
                }
            }
            _ => {
                match match_from_list(text, &mut pos, OPERATORS) {
                    Some(op) => Token::Operator(op),
                    None => return Err(format!("Unexpected token at character index {}.", pos))
                }
            }
        });
    }
    Ok(res)
}

#[test]
fn lex_test_1() {
    use crate::lexer::Token::*;
    use crate::lexer::Operator::*;
    let res = crate::lexer::parse_tokens("let x := [7,22, 3]".as_bytes()).unwrap();
    let expected = [
        Let, 
        Identifier(vec![b'x']), 
        Operator(DefineEqual), 
        OpenBracket, 
        Literal(vec![b'7']), 
        Comma, 
        Literal(vec![b'2', b'2']), 
        Comma, 
        Literal(vec![b'3']), 
        CloseBracket
    ];
    assert!(res == expected);
}
