#[cfg(test)]
mod tests {
    use logos::Logos;

    use tungsten_parser::tokens::Token;

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
