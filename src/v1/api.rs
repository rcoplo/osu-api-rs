use reqwest::Client;
use serde_json::{json, Value};
use serde_json::error::Category::Data;
use crate::entity_v1::{Beatmap, GameRecord, Games, MatchRoom, Replay, Scores, User};
use crate::util::{data_serialize, data_serialize_vec, DataType, UserType};


/// 父url
pub static OSU_API_1: &'static str = "https://osu.ppy.sh/api";
/// reqwest
async fn get(url: String) -> String {
    let client = Client::new();
    let res = client.get(format!("{}{}", OSU_API_1, url))
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .send().await;
    match res {
        Ok(response) => {
            response.text().await.unwrap_or(String::from(""))
        }
        Err(err) => {panic!("数据获取错误: {}", err)}
    }
}

/// # ApiV1
/// 在 https://osu.ppy.sh/p/api 申请一个API KEY，信息随意填写。
///
/// 限制：每分钟1200次，最高瞬时1400次。
///
/// 共有的父URL：https://osu.ppy.sh/api/
pub struct ApiV1{
    /// API KEY
    api_key:String,
}

impl ApiV1 {
    /// ```
    /// use osu_api_rs::api::ApiV1;
    /// use osu_api_rs::ApiV1;
    ///
    /// let api_v1 = ApiV1::new(format!("{}", API_KEY));
    /// // 或
    /// let api_v1 = ApiV1::new(API_KEY);
    /// ```

    pub fn new(api_key:impl Into<String>) -> ApiV1 {
        ApiV1{
            api_key:api_key.into(),
        }
    }
    /// # /api/get_beatmaps
    /// # 谱面信息
    ///
    /// 引用: https://docs.osuwiki.cn/jin-jie-zhi-lu/wei-rao-osu-kaifa#5.1.1-pu-mian-xin-xi
    /// # 参数：
    /// * k - api key （必须）
    /// * since - 返回在该日期之后ranked的所有谱面。必须是MySQL格式。
    /// * s - 指定一个谱面的SetID.
    /// * b - 指定一个谱面的Beatmap ID.
    /// * u - 指定一个用户名/用户数字id.
    /// * type - 指定u参数是数字id还是用户名。对于数字id，该参数值为id，而用户名则参数值为string。默认为智能识别，在纯数字用户名时可能出现问题。
    /// * m - 模式 (0 = osu!, 1 = Taiko, 2 = CtB, 3 = osu!mania). 默认返回所有模式的谱面。
    /// * a - 指定是否包括被转换的谱面（？） (0 = 不包括, 1 = 包括). 只在包含了m参数，并且不为0的情况下有效.被转换的谱面显示它们转换后的难度。默认为0。
    /// * h - 谱面哈希值。举个栗子，如果你尝试获取某个rep对应的谱面，而osr文件只包含谱面的哈希值。(例子：a5b99395a42bd55bc5eb1d2411cbdf8b). 默认情况下， 返回的谱面与Hash值无关。
    /// * limit - 返回值的数量. 默认值（同样是最大值）是500。
    ///
    /// return ：一个包含所有符合指定条件的、ranked谱面的JSON列表。每个难度一个列表。
    /// # Example
    /// ```
    /// use osu_api_rs::ApiV1;
    ///
    /// let api_v1 = ApiV1::new(format!("{}", API KEY));
    /// let beatmaps = api_v1.get_beatmaps(None,Some(3020923),None,None,true,None,None).await;
    /// println!("{:?}", beatmaps);
    /// ```

    pub async fn get_beatmaps(
        &self,
        /*
         set_id 相当于谱的总id
         https://osu.ppy.sh/beatmapsets/1730502#mania/3536583
                                 ^
        */
        set_id:Option<i64>,
        /*
         beatmap_id 相当于谱里面各个难度的id
         https://osu.ppy.sh/beatmapsets/1730502#mania/3536583
                                                         ^
         */
        beatmap_id:Option<i64>,
        // 指定userId或者userName
        user:Option<UserType<'_>>,
        // 游戏模式, (0 = osu!, 1 = Taiko, 2 = CtB, 3 = osu!mania)
        mode:Option<i8>,
        // 是否为转谱
        transformation:bool,
        // hash值 上面有说明
        hash:Option<&str>,
        // 返回的数量
        limit:Option<i16>,
    ) -> Vec<Beatmap> {
       let mut vec =  match user {
            None => {
               vec![
                    ("s", DataType::Int64(set_id)),
                    ("b", DataType::Int64(beatmap_id)),
                    ("m", DataType::Int8(mode)),
                    ("h", DataType::String(hash)),
                    ("limit", DataType::Int16(limit)),
                ]
            }
            Some(user) => {
                match user {
                    UserType::USERID(id) => {
                         vec![
                            ("u", DataType::Int64(Some(id))),
                            ("s", DataType::Int64(set_id)),
                            ("b", DataType::Int64(beatmap_id)),
                            ("m", DataType::Int8(mode)),
                            ("type", DataType::String(Some("id"))),
                            ("h", DataType::String(hash)),
                            ("limit", DataType::Int16(limit)),
                        ]
                    }
                    UserType::USERNAME(name) => {
                        vec![
                            ("u", DataType::String(Some(name))),
                            ("s", DataType::Int64(set_id)),
                            ("b", DataType::Int64(beatmap_id)),
                            ("m", DataType::Int8(mode)),
                            ("type",DataType::String(Some("string"))),
                            ("h", DataType::String(hash)),
                            ("limit", DataType::Int16(limit)),
                        ]
                    }
                }
            }
        };
        if transformation {
            vec.push(("a", DataType::Int8(Some(1))));
        } else {
            vec.push(("a", DataType::Int8(Some(0))));
        }
        let data = get(format!("/get_beatmaps?k={}{}", self.api_key, url(vec))).await;
        data_serialize_vec(data)
    }
    /// 使用 beatmap_id 获取铺面信息
    pub async fn get_beatmap(&self, beatmap_id:Option<i64>,) -> Beatmap {
        let vec = self.get_beatmaps(None, beatmap_id, None, None, true, None, Some(1)).await;
        vec[0].clone()
    }

    /// 使用 set_id 获取铺面信息
    pub async fn get_beatmap_set_id(&self, set_id:Option<i64>,) -> Vec<Beatmap> {
        let vec = self.get_beatmaps(set_id, None, None, None, true, None, None).await;
        vec
    }
     /// 使用 beatmap_id / user 获取铺面信息
    pub async fn get_beatmap_user(&self, beatmap_id:Option<i64>, user_type:Option<UserType<'_>>) -> Beatmap{
        let vec = self.get_beatmaps(None, beatmap_id, user_type, None, true, None, Some(1)).await;
        vec[0].clone()
    }
    /// # /api/get_user
    /// 玩家信息
    ///
    /// 引用: https://docs.osuwiki.cn/jin-jie-zhi-lu/wei-rao-osu-kaifa#5.1.2-wan-jia-xin-xi
    /// # 参数：
    /// * k - api key （必须）
    /// * u - 指定一个用户名/用户数字id。
    /// * m - 模式(0 = osu!, 1 = Taiko, 2 = CtB, 3 = osu!mania).默认值为0。
    /// * type - 指定u参数是数字id还是用户名。对于数字id，该参数值为id，而用户名则参数值为string。默认为智能识别，在纯数字用户名时可能出现问题。
    /// * event_days - 打出最后成绩的日期（last event date），距离现在的最大天数。取值范围为1-31，默认值为1。 【实际使用时好像没有用。同时指定u参数和该参数，和直接指定u参数没有区别。单指定该参数没有返回。】
    ///
    /// 返回值： 包含用户信息的JSON列表。
    /// # Example
    /// ```
    /// use osu_api_rs::{ApiV1, UserType};
    ///
    /// let api_v1 = ApiV1::new(format!("{}", API KEY));
    /// let user = api_v1.get_user(UserType::USERID(18267600), Some(3), None).await;
    /// println!("{:?}", user);
    /// ```
    pub async fn get_user(
        &self,
        // 指定userId或者userName
        user:UserType<'_>,
        // 游戏模式, (0 = osu!, 1 = Taiko, 2 = CtB, 3 = osu!mania)
        mode:Option<i8>,
        // 最后成绩的日期,默认 1
        event_days:Option<i8>
    ) -> User {
       let vec =  match user {
           UserType::USERID(id) => {
               vec![
                   ("u",DataType::Int64(Some(id))),
                   ("m",DataType::Int8(mode)),
                   ("type",DataType::String(Some("id"))),
                   ("event_days",DataType::Int8(event_days)),
               ]
           }
           UserType::USERNAME(name) => {
               vec![
                   ("u",DataType::String(Some(name))),
                   ("m",DataType::Int8(mode)),
                   ("type",DataType::String(Some("string"))),
                   ("event_days",DataType::Int8(event_days)),
               ]
           }
       };
        let data = get(format!("/get_user?k={}{}", self.api_key, url(vec))).await;
        let serialize_vec:Vec<User> = data_serialize_vec(data);
        serialize_vec[0].clone()
    }

    /// # /api/get_scores
    /// # 按谱面获取成绩
    /// 引用: https://docs.osuwiki.cn/jin-jie-zhi-lu/wei-rao-osu-kaifa#5.1.3-an-pu-mian-huo-qu-cheng-ji
    ///
    /// # 参数：
    ///* k - api key （必须）
    ///* b - 指定一个谱面的Beatmap ID。（/b/xxxxx）（必须）
    ///* u - 指定一个要返回分数的用户名/用户数字id。
    ///* m - 模式 (0 = osu!, 1 = Taiko, 2 = CtB, 3 = osu!mania)，默认为0。
    ///* mods -指定一个或者一些mod (具体枚举映射参见后文)
    ///* type - 指定u参数是数字id还是用户名。对于数字id，该参数值为id，而用户名则参数值为string。默认为智能识别，在纯数字用户名时可能出现问题。
    ///* limit - 返回值的数量. 默认值是50，最大值是100。
    ///
    /// 返回值：包含选定谱面前100分数信息的JSON列表。
    /// # Example
    ///```
    ///
    /// use osu_api_rs::{ApiV1, UserType};
    ///
    /// let api_v1 = ApiV1::new(format!("{}", API KEY));
    /// let user = api_v1.get_scores(Some(992512),UserType::USERID(18267600),Some(3),Some(1)).await;
    /// println!("{:?}", user);
    /// ```
    pub async fn get_scores(
        &self,
        // beatmap_id 铺面指定难度id
        beatmap_id: Option<i64>,
        // 指定userId 或者 userName
        user:UserType<'_>,
        // 游戏模式, (0 = osu!, 1 = Taiko, 2 = CtB, 3 = osu!mania)
        mode:Option<i8>,
        // 获取数量
        limit:Option<i8>
    ) -> Vec<Scores> {
        let vec =  match user {
            UserType::USERID(id) => {
                vec![
                    ("u", DataType::Int64(Some(id))),
                    ("b", DataType::Int64(beatmap_id)),
                    ("m", DataType::Int8(mode)),
                    ("type", DataType::String(Some("id"))),
                    ("limit", DataType::Int8(limit)),
                ]
            }
            UserType::USERNAME(name) => {
                vec![
                    ("u", DataType::String(Some(name))),
                    ("b", DataType::Int64(beatmap_id)),
                    ("m", DataType::Int8(mode)),
                    ("type",DataType::String(Some("string"))),
                    ("limit", DataType::Int8(limit)),
                ]
            }
        };
        let data = get(format!("/get_scores?k={}{}", self.api_key, url(vec))).await;
        data_serialize_vec(data)
    }
    pub async fn get_score(
        &self,
        beatmap_id: Option<i64>,
        user:UserType<'_>,
        mode:Option<i8>,
    ) ->Scores {
        let vec = self.get_scores(beatmap_id, user, mode, Some(1)).await;
        vec[0].clone()
    }
    /// # /api/get_user_best
    /// # 玩家的BP
    /// 引用: https://docs.osuwiki.cn/jin-jie-zhi-lu/wei-rao-osu-kaifa#5.1.4-wan-jia-de-bp
    ///
    /// # 参数
    ///* k - api key （必须）
    ///* u - 指定一个要返回分数的用户名/用户数字id。
    ///* m - 模式 (0 = osu!, 1 = Taiko, 2 = CtB, 3 = osu!mania)，默认为0。
    ///* limit - 返回值的数量. 默认值是10，最大值是100。
    ///* type - 指定u参数是数字id还是用户名。对于数字id，该参数值为id，而用户名则参数值为string。默认为智能识别，在纯数字用户名时可能出现问题。
    ///
    /// 返回值：包含了指定用户的BP前10的JSON列表。
    /// # Example
    ///```
    /// use osu_api_rs::{ApiV1, UserType};
    ///
    /// let api_v1 = ApiV1::new(format!("{}", API KEY));
    /// let bp_list = api_v1.get_user_bp_list(UserType::USERID(18267600),Some(3),Some(1)).await;
    /// println!("{:?}", bp_list);
    /// ```
    pub async fn get_user_bp_list(
        &self,
        user:UserType<'_>,
        mode:Option<i8>,
        limit:Option<i8>
    ) -> Vec<GameRecord>{
        let vec =  match user {
            UserType::USERID(id) => {
                vec![
                    ("u", DataType::Int64(Some(id))),
                    ("m", DataType::Int8(mode)),
                    ("type", DataType::String(Some("id"))),
                    ("limit", DataType::Int8(limit)),
                ]
            }
            UserType::USERNAME(name) => {
                vec![
                    ("u", DataType::String(Some(name))),
                    ("m", DataType::Int8(mode)),
                    ("type",DataType::String(Some("string"))),
                    ("limit", DataType::Int8(limit)),
                ]
            }
        };
        let data = get(format!("/get_user_best?k={}{}", self.api_key, url(vec))).await;
        data_serialize_vec(data)
    }
    /// 获取指定 Bp
    pub async fn get_user_bp(
        &self,
        user:UserType<'_>,
        mode:Option<i8>,
        num:i8
    ) -> GameRecord {
        let vec = self.get_user_bp_list(user, mode, Some(num)).await;
        vec[num as usize - 1].clone()
    }
    /// # /api/get_user_recent
    /// # 玩家最近的游戏记录
    ///  引用: https://docs.osuwiki.cn/jin-jie-zhi-lu/wei-rao-osu-kaifa#5.1.5-wan-jia-zui-jin-de-you-xi-ji-lu
    /// # 参数:
    /// * 与获取BP一样，只不过limit的最大值是50。
    /// * 返回值：包含玩家最近10次游戏记录的JSON列表。
    /// * 字段与BP一致，不再赘述
    pub async fn get_user_recent_list(
        &self,
        user:UserType<'_>,
        mode:Option<i8>,
        limit:Option<i8>
    ) -> Vec<GameRecord> {
        let vec =  match user {
            UserType::USERID(id) => {
                vec![
                    ("u", DataType::Int64(Some(id))),
                    ("m", DataType::Int8(mode)),
                    ("type", DataType::String(Some("id"))),
                    ("limit", DataType::Int8(limit)),
                ]
            }
            UserType::USERNAME(name) => {
                vec![
                    ("u", DataType::String(Some(name))),
                    ("m", DataType::Int8(mode)),
                    ("type",DataType::String(Some("string"))),
                    ("limit", DataType::Int8(limit)),
                ]
            }
        };
        let data = get(format!("/get_user_recent?k={}{}", self.api_key, url(vec))).await;
        data_serialize_vec(data)
    }
    /// 获取最新游戏记录,(包括失败)?
    pub async fn get_user_recent(
        &self,
        user:UserType<'_>,
        mode:Option<i8>,
    ) -> GameRecord {
        let vec = self.get_user_recent_list(user, mode, Some(1)).await;
        vec[0].clone()
    }
    /// # /api/get_match
    /// # MP房间信息
    /// 返回一把mp的历史记录。
    ///
    /// # 参数：
    /// * k - api key （必须）
    /// * mp - 房间id（必须）【也就是官网MP Link的参数】
    /// * 在房间中使用 !mp settings 获取match
    ///
    /// 例: https://osu.ppy.sh/community/matches/105537044
    ///
    ///                                             ^
    ///
    /// 返回值：包括房间信息和玩家成绩的JSON列表
    /// # Example
    /// ```
    /// use osu_api_rs::{ApiV1, UserType};
    ///
    /// let api_v1 = ApiV1::new(format!("{}", API KEY));
    /// let room = api_v1.get_match(Some(105537044)).await;
    /// println!("{:?}", room);
    /// ```
    pub async fn get_match(
        &self,
        mp_id:Option<i64>,
    ) -> MatchRoom {
        let vec = vec![
            ("mp", DataType::Int64(mp_id))
        ];
        let data = get(format!("/get_match?k={}{}", self.api_key, url(vec))).await;
        data_serialize(data)
    }
    /// 获取mp最新的成绩
    pub async fn get_match_recent_scores(
        &self,
        mp_id:Option<i64>,
    ) -> Games {
        let match_room = self.get_match(mp_id).await;
        let vec = &match_room.games;
        return if match_room.games.len() != 0 {
            vec[match_room.games.len() - 1].clone()
        } else {
            vec[0].clone()
        }
    }
    /// # /api/get_replay
    /// # 获取回放
    /// 引用: https://docs.osuwiki.cn/jin-jie-zhi-lu/wei-rao-osu-kaifa#5.1.7-huo-qu-hui-fang
    /// # 参数
    ///* k - api key （必须）
    ///* m - 模式 (0 = osu!, 1 = Taiko, 2 = CtB, 3 = osu!mania)，默认为0。（必须）
    ///* b - 指定谱面id（注意！不是BeatmapSet ID，也就是说不是/s/xxxxx而是/b/xxxxx）（必须）
    ///* u - 指定玩家。（必须）
    ///
    /// 返回值：一个包含"content"值的JSON列表，该值中含有base-64加密的rep。
    pub async fn get_replay(
        &self,
        mode:Option<i8>,
        beatmap_id:Option<i64>,
        user:UserType<'_>,
    ) -> Replay {
       let vec =  match user {
            UserType::USERID(id) => {
                vec![
                    ("m",DataType::Int8(mode)),
                    ("b",DataType::Int64(beatmap_id)),
                    ("u",DataType::Int64(Some(id))),
                    ("type",DataType::String(Some("id"))),
                ]
            }
            UserType::USERNAME(name) => {
                vec![
                    ("m",DataType::Int8(mode)),
                    ("b",DataType::Int64(beatmap_id)),
                    ("u",DataType::String(Some(name))),
                    ("type",DataType::String(Some("string"))),
                ]
            }
        };
        let data = get(format!("/get_replay?k={}{}", self.api_key, url(vec))).await;
        data_serialize(data)
    }

}
pub fn url(vec:Vec<(&str,DataType)>) -> String {
    let mut string = String::new();
    for (k,v) in vec {
        match v {
            DataType::Int64(i) => {
                if let Some(v) = i{
                    string.push_str(format!("&{}={}",k,v).as_str());
                }
            }
            DataType::Int32(i) => {
                if let Some(v) = i{
                    string.push_str(format!("&{}={}",k,v).as_str());
                }
            }
            DataType::Int16(i) => {
                if let Some(v) = i{
                    string.push_str(format!("&{}={}",k,v).as_str());
                }
            }
            DataType::Int8(i) => {
                if let Some(v) = i{
                    string.push_str(format!("&{}={}",k,v).as_str());
                }
            }
            DataType::String(str) => {
                if let Some(v) = str{
                    string.push_str(format!("&{}={}",k,v).as_str());
                }
            }
            DataType::Vec(vec) => {
                if let Some(vec) = vec{
                    string.push_str(format!("&{}={:?}",k,vec).as_str());
                }
            }
        }
    }
    string
}