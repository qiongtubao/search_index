pub mod boxed;
pub trait TokenStream {

}

pub trait Tokenizer<'a>: Sized + Clone {
    type TokenStreamImpl: TokenStream;
    fn token_stream(&self, text: &'a str) -> Self::TokenStreamImpl;
}

