use crate::tokenizer::lib::Tokenizer;

trait BoxedTokenizerTrait {

}
#[derive(Clone)]
struct BoxableTokenizer<A>(A) where
        A: for<'a> Tokenizer<'a> + Send + Sync;

impl<A> BoxedTokenizerTrait for BoxableTokenizer<A>
    where  A: for<'a> Tokenizer<'a> + Send + Sync{

}
pub struct BoxedTokenizer(Box<dyn BoxedTokenizerTrait>);

// 定义 T 是 Into<BoxedTokenizer>
// T: 'static + Send + Sync + for<'a> Tokenizer<'a>
// 然后需要依赖 BoxableTokenizer 实现BoxedTokenizerTrait 才能返回dyn BoxedTokenizerTrait
//
impl<T> From<T> for BoxedTokenizer
    where T: 'static + Send + Sync + for<'a> Tokenizer<'a>
{
    fn from(tokenizer: T) -> Self {
        BoxedTokenizer(Box::new(BoxableTokenizer(tokenizer)))
    }
}