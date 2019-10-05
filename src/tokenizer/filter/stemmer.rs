use rust_stemmers::Algorithm;
use crate::tokenizer::lib::{TokenFilter, TokenStream};

pub enum Language {
    Arabic,
    Danish,
    Dutch,
    English,
    Finnish,
    French,
    German,
    Greek,
    Hungarian,
    Italian,
    Portuguese,
    Romanian,
    Russian,
    Spanish,
    Swedish,
    Tamil,
    Turkish,
}

impl Language {
    fn algorithm(self) -> Algorithm {
        use self::Language::*;
        match self {
            Arabic => Algorithm::Arabic,
            Danish => Algorithm::Danish,
            Dutch => Algorithm::Dutch,
            English => Algorithm::English,
            Finnish => Algorithm::Finnish,
            French => Algorithm::French,
            German => Algorithm::German,
            Greek => Algorithm::Greek,
            Hungarian => Algorithm::Hungarian,
            Italian => Algorithm::Italian,
            Portuguese => Algorithm::Portuguese,
            Romanian => Algorithm::Romanian,
            Russian => Algorithm::Russian,
            Spanish => Algorithm::Spanish,
            Swedish => Algorithm::Swedish,
            Tamil => Algorithm::Tamil,
            Turkish => Algorithm::Turkish,
        }
    }
}
#[derive(Clone)]
pub struct Stemmer {
    stemmer_algorithm: Algorithm,
}

impl Stemmer {
    pub fn new(language: Language) -> Stemmer {
        Stemmer {
            stemmer_algorithm: language.algorithm()
        }
    }
}

impl<TailTokenStream> TokenFilter<TailTokenStream> for Stemmer
where TailTokenStream: TokenStream {
    type ResultTokenStream = StemmerTokenStream<TailTokenStream>;

    fn transform(&self, token_stream: TailTokenStream) -> Self::ResultTokenStream {
        let inner_stemmer = rust_stemmers::Stemmer::create(self.stemmer_algorithm);
        StemmerTokenStream::wrap(inner_stemmer, token_stream)
    }
}

pub struct StemmerTokenStream<TailTokenStream>
    where
        TailTokenStream: TokenStream,
{
    tail: TailTokenStream,
    stemmer: rust_stemmers::Stemmer,
}

impl<TailTokenStream> StemmerTokenStream<TailTokenStream>
    where
        TailTokenStream: TokenStream,
{
    fn wrap(
        stemmer: rust_stemmers::Stemmer,
        tail: TailTokenStream,
    ) -> StemmerTokenStream<TailTokenStream> {
        StemmerTokenStream { tail, stemmer }
    }
}

impl<TailTokenStream> TokenStream for StemmerTokenStream<TailTokenStream>
    where
        TailTokenStream: TokenStream,
{

}