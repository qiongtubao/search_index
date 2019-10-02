
trait BoxedTokenizerTrait {

}

pub struct BoxedTokenizer(Box<dyn BoxedTokenizerTrait>);