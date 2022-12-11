use std::collections::HashMap;
use reqwest::{Client, Url};
use reqwest::header::HeaderMap;
use serde_json::{json, Value};
use crate::entity_v2::{Beatmap, BestBeatmapScores, UserBeatmapScore};
use crate::util::{assembly_data, data_serialize, data_serialize_vec, DataType, Mode, Mods};
use crate::error::{Error, Result};
/// 父url
pub static OSU_API_2: &'static str = "https://osu.ppy.sh/api/v2";
/// 客户端凭据授予 (没有关联用户权限)
///
/// osu官方文档说明: https://osu.ppy.sh/docs/index.html#authorization-code-grant
pub static OSU_API_2_OAUTH: &'static str = "https://osu.ppy.sh/oauth/token";
/// reqwest
async fn get(url: Url,access_token:&String) -> String {
    let client = Client::new();
    let res = client.get(url.to_string())
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
async fn post(url: Url,map:Value,access_token:&String) -> String {
    let client = Client::new();
    let res = client.post(url.to_string())
        .json(&map)
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
        println!("{}",&data);
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
    pub async fn lookup_beatmap_all(&self, checksum:Option<&str>, filename:Option<&str>, beatmap_id:Option<i64>) -> Result<Beatmap>  {
        let mut vec = vec![];
        assembly_data(&[
            ("id",DataType::Int64(beatmap_id)),
            ("checksum",DataType::String(checksum)),
            ("filename",DataType::String(filename)),
        ],&mut vec);
        let url = self.assembly_url("beatmaps/lookup", vec);
        let data = get(url, &self.access_token).await;
        data_serialize(data)
    }
    /// 通过 beatmap ID 查找
    pub async fn lookup_beatmap(&self, beatmap_id:i64) -> Result<Beatmap>  {
        self.lookup_beatmap_all(None,None,Some(beatmap_id)).await
    }
    /// # 获取用户Beatmap分数
    /// ## URL Parameters
    /// * beatmap_id - beatmap ID (铺面的一个难度的id)
    /// * user_id - user id (用户id)
    ///
    /// ## Query Parameters
    ///
    /// * mode - 可选 (游戏模式:  fruits , mania , osu , taiko )
    /// * mods - 可选 (匹配Mod的数组 [["DT","MR"]] )
    ///
    ///
    /// return : UserBeatmapScore
    pub async fn get_user_beatmap_score_complete(&self, beatmap_id:i64,user_id:i64,mode:Option<Mode>,mods:Option<Vec<Mods>>) -> Result<UserBeatmapScore>  {
        let mut vec = vec![];

        assembly_data(&[
            ("mode",DataType::Mode(mode,true)),
            ("mods",DataType::Mods(mods)),
        ],&mut vec);

        let url = self.assembly_url(
            format!("beatmaps/{}/scores/users/{}",beatmap_id,user_id),
            vec);
        println!("{:?}", &url.to_string());
        let data = get(url, &self.access_token).await;
        data_serialize(data)
    }
    /// 通过 beatmap_id  /  user_id 获取用户Beatmap分数
    pub async fn get_user_beatmap_score(&self, beatmap_id:i64,user_id:i64) -> Result<UserBeatmapScore>  {
        self.get_user_beatmap_score_complete(beatmap_id,user_id,None,None).await
    }
    /// # 获取用户Beatmap全部分数
    /// ## URL Parameters
    /// * beatmap_id - beatmap ID (铺面的一个难度的id)
    /// * user_id - user id (用户id)
    ///
    /// ## Query Parameters
    ///
    /// * mode - 可选 (游戏模式:  fruits , mania , osu , taiko )
    /// * mods - 可选 (匹配Mod的数组 [["DT","MR"]] )
    ///
    /// return : Vec\<UserBeatmapScore\>
    pub async fn get_user_beatmap_scores_complete_all(&self, beatmap_id:i64,user_id:i64,mode:Option<Mode>,mods:Option<Vec<Mods>>) -> Result<Vec<UserBeatmapScore>>  {
        let mut vec = vec![];

        assembly_data(&[
            ("mode",DataType::Mode(mode,true)),
            ("mods",DataType::Mods(mods)),
        ],&mut vec);

        let url = self.assembly_url(
            format!("beatmaps/{}/scores/users/{}/all",beatmap_id,user_id),
            vec);
        let data = get(url, &self.access_token).await;
        data_serialize_vec(data)
    }
    /// 通过 beatmap_id  /  user_id 获取用户Beatmap全部分数
    pub async fn get_user_beatmap_scores_all(&self, beatmap_id:i64,user_id:i64) -> Result<Vec<UserBeatmapScore>>  {
        self.get_user_beatmap_scores_complete_all(beatmap_id,user_id,None,None).await
    }
    /// # 返回beatmap的最高得分
    /// ## URL Parameters
    /// * beatmap_id - beatmap ID (铺面的一个难度的id)
    ///
    /// ## Query Parameters
    ///
    /// * mode - 可选 (游戏模式:  fruits , mania , osu , taiko )
    /// * mods - 可选 (匹配Mod的数组 [["DT","MR"]] )
    /// * type - 可选 (Beatmap得分排名类型)
    ///
    /// return : BestBeatmapScores
    pub async fn get_beatmap_score(&self, beatmap_id:i64,mode:Option<Mode>,mods:Option<Vec<Mods>>,beatmap_type:Option<&str>) -> Result<BestBeatmapScores>{
        let mut vec = vec![];

        assembly_data(&[
            ("mode",DataType::Mode(mode,false)),
            ("mods",DataType::Mods(mods)),
            ("type",DataType::String(beatmap_type)),
        ],&mut vec);

        let url = self.assembly_url(
            format!("beatmaps/{}/scores",beatmap_id),
            vec);
        let data = get(url, &self.access_token).await;
        data_serialize(data)
    }

    fn assembly_url<URL:AsRef<str> + std::fmt::Display>(&self, url: URL, vec:Vec<(&str,String)>) -> Url {
        Url::parse_with_params(format!("{}/{}",OSU_API_2,<URL as Into<URL>>::into(url)).as_str(),
                               &vec).unwrap()
    }
}
