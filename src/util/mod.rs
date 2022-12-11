mod r#enum;

use std::process::id;
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

pub fn assembly_user_type(user:UserType<'_>, vec: &mut Vec<(&str, String)>){
    match user {
        UserType::USERID(id) => {
            vec.push(("u", id.to_string()));
            vec.push(("type", "id".to_string()));
        }
        UserType::USERNAME(name) => {
            vec.push(("u", name.to_string()));
            vec.push(("type", "string".to_string()));
        }
    }
}
pub fn assembly_data<'a>(data:&[(&'a str, DataType)], vec: &mut Vec<(&'a str, String)>) {
   for (k,data) in data {
       match data {
           DataType::Int64(i) => {
               if let Some(i) = i {
                   vec.push((k,i.to_string()));
               }
           }
           DataType::Int32(i) => {
               if let Some(i) = i {
                   vec.push((k,i.to_string()));
               }
           }
           DataType::Int16(i) => {
               if let Some(i) = i {
                   vec.push((k,i.to_string()));
               }
           }
           DataType::Int8(i) => {
               if let Some(i) = i {
                   vec.push((k,i.to_string()));
               }
           }
           DataType::String(str) => {
               if let Some(str) = str {
                   vec.push((k,str.to_string()));
               }
           }
           DataType::Vec(v) => {
               if let Some(v) = v {
                   for str in v {
                       vec.push((k,str.to_string()));
                   }

               }
           }
       }
   }
}
