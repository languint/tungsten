use chumsky::prelude::*;

pub fn parser<'src>() -> impl Parser<'src, &'src str, ()> {
    end()
}