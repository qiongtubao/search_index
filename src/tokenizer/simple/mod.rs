use std::str::CharIndices;
use crate::tokenizer::lib::{Tokenizer, TokenStream};
use crate::tokenizer::Token;

#[derive(Clone)]
pub struct SimpleTokenizer;

impl<'a> Tokenizer<'a> for SimpleTokenizer {
    type TokenStreamImpl = SimpleTokenStream<'a>;
    fn token_stream(&self, text: &'a str) -> Self::TokenStreamImpl {
        SimpleTokenStream {
            text,
            chars: text.char_indices(),
            token: Token::default(),
        }
    }
}

pub struct SimpleTokenStream<'a> {
    text: &'a str,
    chars: CharIndices<'a>,
    token: Token,
}

impl<'a> TokenStream for SimpleTokenStream<'a> {

}