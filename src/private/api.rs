

//! 这是osu! 私服的api  (v4.3.2)
//!
//! 文档位置: https://github.com/JKBGL/gulag-api-docs
//!
//! 数据可能不准确,本库使用的 https://osu.ppy.sb  的api数据

/// ``` rust
/// //一般使用默认就行了
/// let api =  Api::default();
/// ```
///
/// ```
/// //想用需要密匙的功能就:
/// let api =  Api::new(API_KEY);
/// ```
///
pub struct Api{
    /// 没什么用,大多数api不需要密匙
    api_key: String,
}

impl Api {
    pub fn new(api_key:String) ->Api{
        Self{
            api_key
        }
    }

}

impl Default for Api {
    fn default() -> Self {
        Self{
            api_key: "".to_string(),
        }
    }
}