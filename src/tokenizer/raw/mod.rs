use crate::tokenizer::lib::{Tokenizer, TokenStream};
use crate::tokenizer::Token;

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
        let token = Token {
            offset_from: 0,
            offset_to: text.len(),
            position: 0,
            text: text.to_string(),
            position_length: 1,
        };
        RawTokenStream {
            token,
            has_token: true,
        }
    }
}

