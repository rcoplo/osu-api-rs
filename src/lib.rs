//! # 注意:
//! 有Option枚举 包裹的参数 都是可选的,没有的话就是必须有
//!

mod v1;
mod v2;
mod util;
mod private;
pub mod error;

pub use v1::{
    entity as entity_v1,
    api::{
        ApiV1
    }
};

pub use util::{
    UserType,Mods,Mode
};

pub use v2::{
    entity as entity_v2,
    api::{
        ApiV2
    }
};
