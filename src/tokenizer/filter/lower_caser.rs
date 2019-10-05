use crate::tokenizer::lib::{TokenFilter, TokenStream};
use crate::tokenizer::Token;
use std::mem;

#[derive(Clone)]
pub struct LowerCaser;

impl<TailTokenStream> TokenFilter<TailTokenStream> for LowerCaser
where
    TailTokenStream: TokenStream,
{
    type ResultTokenStream = LowerCaserTokenStream<TailTokenStream>;

    fn transform(&self, token_stream: TailTokenStream) -> Self::ResultTokenStream {
        LowerCaserTokenStream::wrap(token_stream)
    }
}

pub struct LowerCaserTokenStream<TailTokenStream> {
    buffer: String,
    tail: TailTokenStream,
}

impl<TailTokenStream> TokenStream for LowerCaserTokenStream<TailTokenStream>
    where
        TailTokenStream: TokenStream,
{

}

impl<TailTokenStream> LowerCaserTokenStream<TailTokenStream>
    where
        TailTokenStream: TokenStream,
{
    fn wrap(tail: TailTokenStream) -> LowerCaserTokenStream<TailTokenStream> {
        LowerCaserTokenStream {
            tail,
            buffer: String::with_capacity(100),
        }
    }
}