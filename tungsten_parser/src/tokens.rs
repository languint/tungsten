use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\r\n\f]+")]
enum Token {
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
}

#[cfg(test)]
mod tests {
    use logos::Logos;

    use crate::tokens::Token;

    #[test]
    fn float_parsing() {
        let src = "1.0";

        let lexer = Token::lexer(src);

        let mut tokens = vec![];

        for (token, span) in lexer.spanned() {
            match token {
                Ok(token) => tokens.push(token),
                Err(_) => {
                    panic!("lexer error at {:?}", span);
                }
            }
        }

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Float(1.0));
    }

    #[test]
    fn int_parsing() {
        let src = "123";

        let lexer = Token::lexer(src);

        let mut tokens = vec![];

        for (token, span) in lexer.spanned() {
            match token {
                Ok(token) => tokens.push(token),
                Err(_) => {
                    panic!("lexer error at {:?}", span);
                }
            }
        }

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Integer(123));
    }
}
