use crate::tokenizer::lib::{Tokenizer, TokenStream};
use crate::tokenizer::raw::token::Token;

pub mod token;
#[derive(Clone)]
pub struct RawTokenizer;

pub struct RawTokenStream {
    token: Token,
    has_token: bool,
}

impl TokenStream for RawTokenStream {

}

impl<'a> Tokenizer<'a> for RawTokenizer {
    type TokenStreamImpl = RawTokenStream;

    fn token_stream(&self, text: &'a str) -> Self::TokenStreamImpl {
        unimplemented!()
    }
}

