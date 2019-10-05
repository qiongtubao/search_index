use crate::tokenizer::lib::{TokenFilter, TokenStream};
use crate::tokenizer::Token;

#[derive(Clone)]
pub struct RemoveLongFilter {
    length_limit: usize,
}

impl RemoveLongFilter {
    pub fn limit(length_limit: usize) -> RemoveLongFilter {
        RemoveLongFilter {
            length_limit
        }
    }
}

impl<TailTokenStream> TokenFilter<TailTokenStream> for RemoveLongFilter
where
    TailTokenStream: TokenStream, {
   type ResultTokenStream = RemoveLongFilterStream<TailTokenStream>;
    fn transform(&self, token_stream: TailTokenStream) -> Self::ResultTokenStream  {
        RemoveLongFilterStream::wrap(self.length_limit, token_stream)
    }
}

pub struct RemoveLongFilterStream<TailTokenStream>
    where
        TailTokenStream: TokenStream,
{
    token_length_limit: usize,
    tail: TailTokenStream,
}


impl<TailTokenStream> TokenStream for RemoveLongFilterStream<TailTokenStream>
    where
        TailTokenStream: TokenStream,
{

}
impl<TailTokenStream> RemoveLongFilterStream<TailTokenStream>
    where
        TailTokenStream: TokenStream,
{
    fn predicate(&self, token: &Token) -> bool {
        token.text.len() < self.token_length_limit
    }

    fn wrap(
        token_length_limit: usize,
        tail: TailTokenStream,
    ) -> RemoveLongFilterStream<TailTokenStream> {
        RemoveLongFilterStream {
            token_length_limit,
            tail,
        }
    }
}