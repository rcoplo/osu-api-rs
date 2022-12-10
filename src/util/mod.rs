mod r#enum;

pub use r#enum::{
    UserType,DataType
};



pub fn data_serialize_vec<ApiData :for<'a> serde::Deserialize<'a> + serde::Serialize>(data:String) -> Vec<ApiData> {
    let vec = serde_json::from_str::<Vec<ApiData>>(data.as_str()).unwrap();
    vec
}
pub fn data_serialize<ApiData :for<'a> serde::Deserialize<'a> + serde::Serialize>(data:String) -> ApiData {

    let vec = serde_json::from_str::<ApiData>(data.as_str()).unwrap();
    vec
}

