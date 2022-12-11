//! 所有对象结构: https://osu.ppy.sh/docs/index.html#beatmap
//!
//! 数据就不详细说明了,自己看看文档吧~

use serde_json::Value;
use crate::entity_v1::Scores;

#[derive(Debug, Clone,serde::Serialize,serde::Deserialize)]
pub struct Beatmap {
    pub beatmapset_id:i64,
    pub difficulty_rating:f32,
    pub id:i64,
    pub mode:String,
    pub status:String,
    pub total_length:i32,
    pub user_id:i64,
    pub version:String,
    pub accuracy:i32,
    pub ar:f32,
    pub bpm:i64,
    pub convert:bool,
    pub count_circles:i32,
    pub count_sliders:i32,
    pub count_spinners:i32,
    pub cs:f32,
    pub drain:f32,
    pub hit_length:i32,
    pub is_scoreable:bool,
    pub last_updated:String,
    pub mode_int:i32,
    pub passcount:i32,
    pub playcount:i32,
    pub ranked:i32,
    pub url:String,
    pub checksum:String,
    pub beatmapset:Beatmapset,
    pub failtimes: Failtimes,
    pub max_combo: i32,
}

#[derive(Debug, Clone,serde::Serialize,serde::Deserialize)]
pub struct Beatmapset{
    pub artist:String,
    pub artist_unicode:String,
    pub covers:Covers,
    pub creator:String,
    pub favourite_count:i32,
    pub id:i64,
    pub nsfw:bool,
    pub offset:i32,
    pub play_count:i32,
    pub preview_url:String,
    pub source:String,
    pub spotlight:bool,
    pub status:String,
    pub title:String,
    pub title_unicode:String,
    pub user_id:i64,
    pub video:bool,
    pub availability:Availability,
    pub bpm:i64,
    pub can_be_hyped:bool,
    pub discussion_enabled:bool,
    pub discussion_locked:bool,
    pub is_scoreable:bool,
    pub last_updated:String,
    pub legacy_thread_url:String,
    pub nominations_summary: NominationsSummary,
    pub ranked: i32,
    pub ranked_date: String,
    pub storyboard: bool,
    pub submitted_date: String,
    pub tags: String,
    pub ratings: Vec<i32>,
}

#[derive(Debug, Clone,serde::Serialize,serde::Deserialize)]
pub struct Covers{
    pub cover:String,
    #[serde(rename = "cover@2x")]
    pub cover2x:String,
    pub card:String,
    #[serde(rename = "card@2x")]
    pub card2x:String,
    pub list:String,
    #[serde(rename = "list@2x")]
    pub list2x:String,
    pub slimcover:String,
    #[serde(rename = "slimcover@2x")]
    pub slimcover2x:String,
}

#[derive(Debug, Clone,serde::Serialize,serde::Deserialize)]
pub struct Availability{
    pub download_disabled:bool,

}

#[derive(Debug, Clone,serde::Serialize,serde::Deserialize)]
pub struct NominationsSummary {
    pub current:i32,
    pub required:i32,
}

#[derive(Debug, Clone,serde::Serialize,serde::Deserialize)]
pub struct Failtimes {
    pub fail:Vec<i32>,
    pub exit:Vec<i32>,
}

#[derive(Debug, Clone,serde::Serialize,serde::Deserialize)]
pub struct UserBeatmapScore {
    pub position:i64,
    pub score:Score,

}

#[derive(Debug, Clone,serde::Serialize,serde::Deserialize)]
pub struct Score {
    pub accuracy:f64,
    pub best_id:i64,
    pub created_at:String,
    pub id:i64,
    pub max_combo:i64,
    pub mode:String,
    pub mode_int:i64,
    pub mods:Vec<String>,
    pub passed:bool,
    pub perfect:bool,
    pub pp:f32,
    pub rank:String,
    pub replay:bool,
    pub score:i64,
    pub statistics: Statistics,
    pub r#type:String,
    pub user_id:i64,
    pub current_user_attributes:Value, // 没获取到,不知道里面什么数据
    pub beatmap:ScoreBeatmap,
    pub user:User,
}



#[derive(Debug, Clone,serde::Serialize,serde::Deserialize)]
pub struct ScoreBeatmap {
    pub beatmapset_id:i64,
    pub difficulty_rating:f32,
    pub id:i64,
    pub mode:String,
    pub status:String,
    pub total_length:i32,
    pub user_id:i64,
    pub version:String,
    pub accuracy:i32,
    pub ar:f32,
    pub bpm:i64,
    pub convert:bool,
    pub count_circles:i32,
    pub count_sliders:i32,
    pub count_spinners:i32,
    pub cs:f32,
    pub drain:f32,
    pub hit_length:i32,
    pub is_scoreable:bool,
    pub last_updated:String,
    pub mode_int:i32,
    pub passcount:i32,
    pub playcount:i32,
    pub ranked:i32,
    pub url:String,
    pub checksum:String,
}

#[derive(Debug, Clone,serde::Serialize,serde::Deserialize)]
pub struct Statistics {
    pub count_100:i32,
    pub count_300:i32,
    pub count_50:i32,
    pub count_geki:i32,
    pub count_katu:i32,
    pub count_miss:i32,
}
#[derive(Debug, Clone,serde::Serialize,serde::Deserialize)]
pub struct User {
    pub avatar_url:String,
    pub country_code:String,
    pub default_group:String,
    pub id:i64,
    pub is_active:bool,
    pub is_bot:bool,
    pub is_deleted:bool,
    pub is_online:bool,
    pub is_supporter:bool,
    pub last_visit:String,
    pub pm_friends_only:bool,
    pub username:String,
    pub country:Country,
    pub cover:Cover,

}

#[derive(Debug, Clone,serde::Serialize,serde::Deserialize)]
pub struct Country {
    pub code:String,
    pub name:String,
}

#[derive(Debug, Clone,serde::Serialize,serde::Deserialize)]
pub struct Cover{
    pub url:String,
    pub id:String,
}

#[derive(Debug, Clone,serde::Serialize,serde::Deserialize)]
pub struct BestBeatmapScores {
    pub scores:Vec<BestScores>,
}

#[derive(Debug, Clone,serde::Serialize,serde::Deserialize)]
pub struct BestScores {
    pub accuracy:f64,
    pub best_id:i64,
    pub created_at:String,
    pub id:i64,
    pub max_combo:i32,
    pub mode:String,
    pub mode_int:i32,
    pub mods:Vec<String>,
    pub passed:bool,
    pub perfect:bool,
    pub pp:f32,
    pub rank:String,
    pub replay:bool,
    pub score:i32,
    pub statistics:Statistics,
    pub r#type:String,
    pub user_id:i64,
    pub current_user_attributes:Value,
    pub user:User,
}

