
#[derive(Debug, Serialize, Deserialize)]
pub enum IndexRecordOption {
    ///只记录docId
    #[serde(rename = "basic")]
    Basic,
    ///记录文档ID以及术语频率.
    ///频率一词可以帮助对文档进行更好的评分.
    #[serde(rename = "freq")]
    WithFreqs,
    ///记录文档ID，术语频率和位置
    /// 文档中的出现。
    /// 需要位置才能运行[PhraseQueries]（../ query / struct.PhraseQuery.html）。
    #[serde(rename = "position")]
    WithFreqsAndPositions,
}