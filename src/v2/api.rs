use std::collections::HashMap;
use std::process::id;
use reqwest::{Client, Error, Response};
use reqwest::header::HeaderMap;
use serde_json::{json, Value};
use crate::entity_v2::{Beatmap, UserBeatmapScore};
use crate::util::{data_serialize, data_serialize_vec, DataType};
use crate::v2;

/// 父url
pub static OSU_API_2: &'static str = "https://osu.ppy.sh/api/v2";
/// 客户端凭据授予 (没有关联用户权限)
///
/// osu官方文档说明: https://osu.ppy.sh/docs/index.html#authorization-code-grant
pub static OSU_API_2_OAUTH: &'static str = "https://osu.ppy.sh/oauth/token";
/// reqwest
async fn get(url: String,access_token:&String) -> String {
    let client = Client::new();
    let res = client.get(format!("{}{}", OSU_API_2, url))
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {}",access_token))
        .send().await;
    match res {
        Ok(response) => {
            response.text().await.unwrap_or(String::from(""))
        }
        Err(err) => {panic!("数据获取错误: {}", err)}
    }
}

/// 在 https://osu.ppy.sh/home/account/edit  申请一个新的 OAuth 应用
///
///应用回调链接 随便填写
/// 我们只需要
/// * 客户端 ID
/// * 客户端密钥
///
/// 就可以了
#[derive(Debug, Clone,serde::Serialize,serde::Deserialize)]
/// 结构体里面是 返回的数据
pub struct ApiV2 {
    /// 认证类型: Bearer
    token_type:String,
    /// 到期时间 1天刷新一次
    expires_in:i32,
    /// token
    access_token:String,
}

impl ApiV2 {
    /// # Example
    ///
    /// ```
    /// use osu_api_rs::ApiV2;
    /// let api_v1 = ApiV2::new(client_id,client_secret);
    /// // 或
    /// let api_v1 = ApiV2::new(format!("{}",client_id),format!("{}",client_secret));
    /// ```
    ///
    /// * 客户端凭证  唯一值: client_credentials
    ///
    ///     grant_type:client_credentials
    ///
    /// * 客户端 ID
    ///
    ///     client_id:String
    ///
    /// * 客户端密钥
    ///
    ///     client_secret:String
    ///
    /// * 权限范围 唯一值: public
    ///
    ///     scope:public
    ///

    pub async fn new(client_id:impl Into<String>, client_secret:impl Into<String>) -> ApiV2 {
        let json = json!({
	        "grant_type": "client_credentials",
	        "client_id": client_id.into(),
	        "client_secret": client_secret.into(),
	        "scope": "public"
        });
        let client = Client::new();
        let data = client.post(OSU_API_2_OAUTH)
            .header("Content-Type", "application/json")
            .json(&json).send().await.unwrap().text().await.unwrap();
        serde_json::from_str::<ApiV2>(data.as_str()).unwrap()
    }
    /// # Beatmaps
    /// ## GET /beatmaps/lookup
    /// ## Lookup Beatmap
    ///
    /// ### 查询参数:
    /// * checksum   - 可选  beatmap 的校验?
    /// * filename  - 可选  要查找的文件名。
    /// * id  - 可选   要查找的 beatmap ID。
    ///
    ///
    ///
    pub async fn lookup_beatmap_all(&self, checksum:Option<&str>, filename:Option<&str>, beatmap_id:Option<i64>) -> Beatmap  {
        let vec = vec![
            ("id",DataType::Int64(beatmap_id),true),
            ("checksum",DataType::String(checksum),false),
            ("filename",DataType::String(filename),false),
        ];
        let data = get(format!("/beatmaps/lookup{}", url(vec)), &self.access_token).await;
        data_serialize(data)
    }
    /// 通过 beatmap ID 查找
    pub async fn lookup_beatmap(&self, beatmap_id:i64) -> Beatmap  {
        self.lookup_beatmap_all(None,None,Some(beatmap_id)).await
    }
    /// # 获取用户Beatmap分数
    /// ## URL Parameters
    /// * beatmap_id - beatmap ID (铺面的一个难度的id)
    ///
    /// * user_id - user id (用户id)
    ///
    /// ## Query Parameters
    ///
    /// * mode - 可选 (游戏模式: [fruits,mania,osu,taiko])
    ///
    ///  * mods - 可选 (匹配Mod的数组)
    ///
    pub async fn get_user_beatmap_score_all(&self, beatmap_id:i64,user_id:i64,mode:Option<&str>,mods:Option<Vec<&str>>) -> UserBeatmapScore  {
        let vec = vec![
            ("mode",DataType::String(mode),true),
            ("mods",DataType::Vec(mods),false),
        ];
        let data = get(format!("/beatmaps/{}/scores/users/{}{}",beatmap_id,user_id, url(vec)), &self.access_token).await;
        data_serialize(data)
    }
    /// 通过 beatmap_id  /  user_id 获取用户Beatmap分数
    pub async fn get_user_beatmap_score(&self, beatmap_id:i64,user_id:i64) -> UserBeatmapScore  {
        self.get_user_beatmap_score_all(beatmap_id,user_id,None,None).await
    }

    
}

pub fn url(vec:Vec<(&str,DataType,bool)>) -> String {
    let mut string = String::new();
    for (k,v,b) in vec {
        match v {
            DataType::Int64(i) => {
                if b {
                    if let Some(v) = i{
                        string.push_str(format!("?{}={}",k,v).as_str());
                    }
                }else {
                    if let Some(v) = i{
                        string.push_str(format!("&{}={}",k,v).as_str());
                    }
                }

            }
            DataType::Int32(i) => {
                if b {
                    if let Some(v) = i{
                        string.push_str(format!("?{}={}",k,v).as_str());
                    }
                }else {
                    if let Some(v) = i{
                        string.push_str(format!("&{}={}",k,v).as_str());
                    }
                }
            }
            DataType::Int16(i) => {
                if b {
                    if let Some(v) = i{
                        string.push_str(format!("?{}={}",k,v).as_str());
                    }
                }else {
                    if let Some(v) = i{
                        string.push_str(format!("&{}={}",k,v).as_str());
                    }
                }
            }
            DataType::Int8(i) => {
                if b {
                    if let Some(v) = i{
                        string.push_str(format!("?{}={}",k,v).as_str());
                    }
                }else {
                    if let Some(v) = i{
                        string.push_str(format!("&{}={}",k,v).as_str());
                    }
                }
            }
            DataType::String(str) => {
                if b {
                    if let Some(v) = str{
                        string.push_str(format!("?{}={}",k,v).as_str());
                    }
                }else {
                    if let Some(v) = str{
                        string.push_str(format!("&{}={}",k,v).as_str());
                    }
                }
            }
            DataType::Vec(vec) => {
                if b {
                    if let Some(vec) = vec{
                        string.push_str(format!("?{}={:?}",k,vec).as_str());
                    }
                }else {
                    if let Some(vec) = vec{
                        string.push_str(format!("&{}={:?}",k,vec).as_str());
                    }
                }
            }
        }
    }
    string
}
