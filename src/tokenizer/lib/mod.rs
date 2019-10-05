use crate::tokenizer::chain::ChainTokenizer;

pub mod boxed;
pub trait TokenStream {

}

pub trait Tokenizer<'a>: Sized + Clone {
    type TokenStreamImpl: TokenStream;
    fn token_stream(&self, text: &'a str) -> Self::TokenStreamImpl;
    fn filter<NewFilter>(self, new_filter: NewFilter) -> ChainTokenizer<NewFilter, Self>
        where NewFilter: TokenFilter<<Self as Tokenizer<'a>>::TokenStreamImpl>
    {
        ChainTokenizer {
            head: new_filter,
            tail: self,
        }
    }
}

pub trait TokenFilter<TailTokenStream: TokenStream>: Clone {
    type ResultTokenStream: TokenStream;
    fn transform(&self, token_stream: TailTokenStream) -> Self::ResultTokenStream;
}

