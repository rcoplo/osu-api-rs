mod r#enum;


use serde_json::{json, to_string, Value};
pub use r#enum::{
    UserType,DataType,Mode,Mods
};
use crate::error::{Error, Result};

pub fn data_serialize_vec<ApiData :for<'a> serde::Deserialize<'a> + serde::Serialize>(data:String) -> Result<Vec<ApiData>> {
    let result = serde_json::from_str::<Vec<ApiData>>(data.as_str());
    match result {
        Ok(vec) => {
            return if vec.is_empty(){
                Err(Error::Null)
            } else{
                Ok(vec)
            }
        }
        Err(err) => {
            Err(Error::Error(err))
        }
    }
}

pub fn data_serialize<ApiData :for<'a> serde::Deserialize<'a> + serde::Serialize>(data:String) -> Result<ApiData> {
    let json = serde_json::from_str::<Value>(data.as_str());
    let json= match json {
        Ok(data) => {
            if !data.get("error").is_none() && data["error"].is_null() {
                return Err(Error::Null);
            }else {
                Ok(data)
            }
        }
        Err(err) => {
            return Err(Error::Error(err));
        }
    };
    match json {
        Ok(data) => {
            let result = serde_json::from_value::<ApiData>(data);
            match result {
                Ok(data) => {
                    Ok(data)
                }
                Err(err) => {
                    Err(Error::Error(err))
                }
            }
        }
        Err(err) => {
            Err(err)
        }
    }
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
           DataType::Mods(v) => {
               if let Some(m) = v {
                   let vec = Mods::get_mods(m);
                   // vec.push((k,format!("{:?}",v)));
               }
           }
           DataType::Mode(m,is_str) => {
               if *is_str {
                   if let Some(m) = m {
                       let (str,_) = Mode::get_mode(m);
                       vec.push((k,format!("{:?}",str)));
                   }
               }else {
                   if let Some(m) = m {
                       let (_,i) = Mode::get_mode(m);
                       vec.push((k,format!("{:?}",i)));
                   }
               }

           }
       }
   }
}

impl Mods {
    pub fn get_mods<'a>(mods:&Vec<Mods>) -> Vec<(i32,&'a str)> {
        let mut vec = vec![];
        for m in mods {
            match m {
                Mods::NONE => vec.push((0, "NONE")),
                Mods::NF => vec.push((1, "NF")),
                Mods::EZ => vec.push((2, "EZ")),
                Mods::NV => vec.push((4, "NV")),
                Mods::MR => vec.push((0, "MR")),
                Mods::HD => vec.push((8, "HD")),
                Mods::HR => vec.push((16, "HR")),
                Mods::SD => vec.push((32, "SD")),
                Mods::DT => vec.push((64, "DT")),
                Mods::RX => vec.push((128, "RX")),
                Mods::HT => vec.push((256, "HT")),
                Mods::NC => vec.push((512, "NC")),
                Mods::FL => vec.push((1024, "FL")),
                Mods::AUTO => vec.push((2048, "Auto")),
                Mods::SO => vec.push((4096, "SO")),
                Mods::AP => vec.push((8192, "AP")),
                Mods::PF => vec.push((16384, "PF")),
                Mods::Key1 => vec.push((67108864,"1K")),
                Mods::Key2 => vec.push((268435456,"2K")),
                Mods::Key3 => vec.push((134217728,"3K")),
                Mods::Key4 => vec.push((32768,"4K")),
                Mods::Key5 => vec.push((65536,"5K")),
                Mods::Key6 => vec.push((131072,"6K")),
                Mods::Key7 => vec.push((262144,"7K")),
                Mods::Key8 => vec.push((524288,"8K")),
                Mods::Key9 => vec.push((2,"9K")),
                Mods::Key10 => vec.push((2,"10K")),
                Mods::FadeIn => vec.push((1048576,"FadeIn")),
                Mods::Random => vec.push((2097152,"Random")),
                Mods::Cinema => vec.push((4194304,"Cinema")),
                Mods::Coop => vec.push((33554432, "Coop")),
                Mods::V2 => vec.push((536870912, "V2")),
                Mods::LM => vec.push((1073741824, "LM")),
                Mods::KeyMod => {
                    // vec.push((2,"KeyMod"));
                },
                Mods::FreeModAllowed => {
                    // vec.push((2,"FreeModAllowed"))
                }
            }
        }
        vec
    }
    pub fn str_to_mods(str:Vec<&str>) -> Vec<Mods>{
        let mut vec = vec![];
        for s in str {
            match  s{
                "NONE" => vec.push(Mods::NONE),
                "NF" => vec.push(Mods::NF),
                "EZ" => vec.push(Mods::EZ),
                "NV" => vec.push(Mods::NV),
                _ => {}
            }
        }
        vec
    }
}


impl Mode {
    pub fn get_mode<'a>(mode:&Mode) -> (&'a str,i8) {
        match mode {
            Mode::Fruits => {
                ("fruits",2)
            }
            Mode::Mania => {
                ("mania",3)
            }
            Mode::Osu => {
                ("osu",0)
            }
            Mode::Taiko => {
                ("taiko",1)
            }
        }
    }
}