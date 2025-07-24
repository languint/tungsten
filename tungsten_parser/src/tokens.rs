use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    Error,

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{{")]
    LBrace,
    #[token("}}")]
    RBrace,

    #[regex(r"[1-9][0-9]*\.[0-9]*", |lex| lex.slice().parse::<f64>().unwrap())]
    #[regex(r"0\.[0-9]+", |lex| lex.slice().parse::<f64>().unwrap())]
    Float(f64),

    #[regex("[0-9]+", |lex| lex.slice().parse::<isize>().unwrap())]
    Integer(isize),

    #[regex(r"[ \t\f\n]+", logos::skip)]
    Whitespace,
}

