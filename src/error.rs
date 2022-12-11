
/// 错误权举
#[derive(Debug)]
pub enum Error {
    /// 获取的数据为空
    Null,
    /// 表示序列化或反序列化JSON数据时可能发生的所有错误。
    Error(serde_json::error::Error),
}

pub type Result<T> = std::result::Result<T,Error>;