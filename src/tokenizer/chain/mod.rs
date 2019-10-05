use crate::tokenizer::lib::{Tokenizer, TokenFilter};

#[derive(Clone)]
pub struct ChainTokenizer<HeadTokenFilterFactory, TailTokenizer> {
    pub head: HeadTokenFilterFactory,
    pub tail: TailTokenizer,
}

impl<'a, HeadTokenFilterFactory, TailTokenizer> Tokenizer<'a>
    for ChainTokenizer<HeadTokenFilterFactory, TailTokenizer>
where
    HeadTokenFilterFactory: TokenFilter<TailTokenizer::TokenStreamImpl>,
    TailTokenizer: Tokenizer<'a>,
{
    type TokenStreamImpl = HeadTokenFilterFactory::ResultTokenStream;
    fn token_stream(&self, text: &'a str) -> Self::TokenStreamImpl {
        let tail_token_stream = self.tail.token_stream(text);
        self.head.transform(tail_token_stream)
    }
}

