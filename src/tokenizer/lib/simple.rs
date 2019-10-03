
#[derive(Clone)]
pub struct SimpleTokenizer;

pub struct SimpleTokenStream<'a> {
    text: &'a str,
    chars: CharIndices<'a>,
    token: Token,
}

impl<'a> Tokenizer<'a> for SimpleTokenizer {
    type TokenStreamImpl = SimpleTokenStream<'a>;
    
}