///  # Beatmap
/// 引用: https://docs.osuwiki.cn/jin-jie-zhi-lu/wei-rao-osu-kaifa#5.1.1-pu-mian-xin-xi
///
///数据以下面结构体为准
/// ```  json
///
/// [
/// // 4 = loved, 3 = qualified, 2 = approved, 1 = ranked, 0 = pending, -1 = WIP, -2 = graveyard
/// {"approved" : "1",
///  // ranked日期, 时区为UTC+8
/// "approved_date" : "2013-07-02 01:01:12",
/// // 最后更新日期，时区同上。 如果谱面被Unranked之后Reranked，该日期可能晚于上面的日期。
///  "last_update" : "2013-07-06 16:51:22",
///  "artist" : "Luxion",
///  "beatmap_id" : "252002", // 每个难度的Beatmap_ID
///  "beatmapset_id" : "93398", // 所有难度的BeatmapSet_ID
///  "bpm" : "196",
///  "creator" : "RikiH_",
///  "difficultyrating" : "5.59516", // 游戏中和网站上的难度星数
/// "diff_size" : "4", // CS
/// "diff_overall" : "6", // OD
/// "diff_approach" : "7", // AR
/// "diff_drain" : "6", // HP
/// "hit_length" : "113", // 第一个note到最后一个note的间隔（不计算breaks）
/// "source" : "BMS",
/// // 0 = any, 1 = unspecified, 2 = video game, 3 = anime, 4 = rock, 5 = pop, 6 = other, 7 = novelty, 9 = hip hop, 10 = electronic (note that there's no 8)
/// "genre_id" : "1",
/// // 0 = any, 1 = other, 2 = english, 3 = japanese, 4 = chinese, 5 = instrumental, 6 = korean, 7 = french, 8 = german, 9 = swedish, 10 = spanish, 11 = italian
/// "language_id" : "5",
/// "title" : "High-Priestess", // 歌曲名称
/// "total_length" : "145", // 第一个note到最后一个note的间隔（计算breaks）
/// "version" : "Overkill", // 难度名
/// "file_md5" : "c8f08438204abfcdd1a748ebfae67421", // 谱面文件（不知道是.osz还是.osu）的MD5哈希值
/// "mode" : "0", // 模式,
/// "tags" : "melodious long", // tags
/// // 被收藏的次数。特别的，该条目采用英式英语，而不是美式英语（favorite_count）
/// "favourite_count" : "121",
/// "playcount" : "9001", // 被玩（。）的次数
/// "passcount" : "1337", // 被pass的次数（玩家没有fail或者retry）
/// "max_combo" : "2101" // 最大combo数
/// }, { ... }, ...]
///
/// ```
#[derive(Debug, Clone,serde::Serialize,serde::Deserialize)]
pub struct Beatmap {
    pub beatmapset_id:String,
    pub beatmap_id:String,
    pub approved:String,
    pub total_length:String,
    pub hit_length:String,
    pub version:String,
    pub file_md5:String,
    pub diff_size:String,
    pub diff_overall:String,
    pub diff_approach:String,
    pub diff_drain:String,
    pub mode:String,
    pub count_normal:String,
    pub count_slider:String,
    pub count_spinner:String,
    pub submit_date:String,
    pub approved_date:String,
    pub last_update:String,
    pub artist:String,
    pub artist_unicode:String,
    pub title:String,
    pub title_unicode:String,
    pub creator:String,
    pub creator_id:String,
    pub bpm:String,
    pub source:String,
    pub tags:String,
    pub genre_id:String,
    pub language_id:String,
    pub favourite_count:String,
    pub rating:String,
    pub storyboard:String,
    pub video:String,
    pub download_unavailable:String,
    pub audio_unavailable:String,
    pub playcount:String,
    pub passcount:String,
    pub packs:String,
    pub max_combo:String,
    pub difficultyrating:String,
}

///  # User
/// 引用: https://docs.osuwiki.cn/jin-jie-zhi-lu/wei-rao-osu-kaifa#5.1.2-wan-jia-xin-xi
///
/// 数据以下面结构体为准
/// ```  json
/// [{"user_id" : "1",
///"username" : "User name",
/// "count300" : "1337",
/// "count100" : "123",
/// "count50" : "69",
/// "playcount" : "42", // 以上四条为所有Ranked，Approved，Loved谱面中的计数
/// "ranked_score" : "666666", // 所有Ranked，Approved，Loved谱面中的最高分计数
/// "total_score" : "999999998", // 所有Ranked，Approved，Loved谱面中所有成绩的总分。
/// "pp_rank" : "2442",
/// "level" : "50.5050",
/// "pp_raw" : "3113", // 不活跃的玩家PP值是0，以将他们排除出PP榜单
/// "accuracy" : "98.1234",
/// "count_rank_ss": "54",
/// "count_rank_s" : "81", // 获得的SS S A的数目
/// "count_rank_a" : "862",
/// "country" : "DE", // 使用ISO3166-1 alpha-2国家命名规范。 参考http://en.wikipedia.org/wiki/ISO_3166-1_alpha-2/wiki/ISO_3166-1_alpha-2
/// "pp_country_rank":"1337", // 国内的PP排名
/// "events" : [{ // 记录这个用户上传的成绩【但是实际应用并没有获取到】
/// "display_html": "<img src='\/images\/A_small.png'\/>...",//评级对应的图片
/// "beatmap_id": "222342",
/// "beatmapset_id": "54851",
/// "date": "2013-07-07 22:34:04",
/// "epicfactor": "1" // 这个成绩有多么的“史诗级”，取值范围是1-32  貌似api里面已经没了
/// }, { ... }, ...]
/// ```
///
#[derive(Debug, Clone,serde::Serialize,serde::Deserialize)]
pub struct User {
    pub user_id:String,
    pub username:String,
    pub join_date:String,
    pub count300:String,
    pub count100:String,
    pub count50:String,
    pub playcount:String,
    pub ranked_score:String,
    pub total_score:String,
    pub pp_rank:String,
    pub level:String,
    pub pp_raw:String,
    pub accuracy:String,
    pub count_rank_ss:String,
    pub count_rank_ssh:String,
    pub count_rank_s:String,
    pub count_rank_sh:String,
    pub count_rank_a:String,
    pub country:String,
    pub total_seconds_played:String,
    pub pp_country_rank:String,
    pub events:Vec<String>,
}

///  # Scores
/// 引用: https://docs.osuwiki.cn/jin-jie-zhi-lu/wei-rao-osu-kaifa#5.1.3-an-pu-mian-huo-qu-cheng-ji
///
/// 数据以下面结构体为准
/// ```  json
/// [{"score" : "1234567",
/// "username" : "User name",
/// "count300" : "300",
/// "count100" : "50",
/// "count50" : "10",
/// "countmiss" : "1",
/// "maxcombo" : "321",
/// "countkatu" : "10",
/// "countgeki" : "50",
/// "perfect" : "0", // 只有取得了地图的最大cb时，该值为1
/// "enabled_mods" : "76", // 具体参见枚举
/// "user_id" : "1",
/// "date" : "2013-06-22 9:11:16",
/// "rank" : "SH",
/// "pp" : "1.3019" //4位小数
/// },
///
/// {...},
///
/// ...]
/// ```
///
#[derive(Debug, Clone,serde::Serialize,serde::Deserialize)]
pub struct Scores{
    pub score_id :String,
    pub score :String,
    pub username :String,
    pub maxcombo :String,
    pub count50 :String,
    pub count100 :String,
    pub count300 :String,
    pub countmiss :String,
    pub countkatu :String,
    pub countgeki :String,
    pub perfect :String,
    pub enabled_mods :String,
    pub user_id :String,
    pub date :String,
    pub rank :String,
    pub pp :String,
    pub replay_available :String,
}
#[derive(Debug, Clone,serde::Serialize,serde::Deserialize)]
pub struct GameRecord {
    pub beatmap_id :String,
    pub score_id :String,
    pub score :String,
    pub maxcombo :String,
    pub count50 :String,
    pub count100 :String,
    pub count300 :String,
    pub countmiss :String,
    pub countkatu :String,
    pub countgeki :String,
    pub perfect :String,
    pub enabled_mods :String,
    pub user_id :String,
    pub date :String,
    pub rank :String,
    pub pp :String,
    pub replay_available :String,
}
#[derive(Debug, Clone,serde::Serialize,serde::Deserialize)]
pub struct MatchRoom {
    #[serde(rename = "match")]
    pub room_info: RoomInfo,
    pub games :Vec<Games>,
}

#[derive(Debug, Clone,serde::Serialize,serde::Deserialize)]
pub struct RoomInfo {
    pub match_id :String,
    pub name :String,
    pub start_time :String,
    pub end_time :Option<String>,
}

#[derive(Debug, Clone,serde::Serialize,serde::Deserialize)]
pub struct Games {
    pub game_id :String,
    pub start_time :String,
    pub end_time :String,
    pub beatmap_id :String,
    pub play_mode :String,
    pub match_type :String,
    pub scoring_type :String,
    pub team_type :String,
    pub mods :String,
    pub scores :Vec<RoomScores>,
}
#[derive(Debug, Clone,serde::Serialize,serde::Deserialize)]
pub struct RoomScores {
    pub slot :String,
    pub team :String,
    pub user_id :String,
    pub score :String,
    pub maxcombo :String,
    pub rank :String,
    pub count50 :String,
    pub count100 :String,
    pub count300 :String,
    pub countmiss :String,
    pub countgeki :String,
    pub countkatu :String,
    pub perfect :String,
    pub pass :String,
    pub enabled_mods :Option<String>,
}
