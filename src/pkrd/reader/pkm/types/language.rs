use alloc::{
    format,
    string::{String, ToString},
};
use num_enum::FromPrimitive;

#[derive(Clone, Copy, Debug, Eq, PartialEq, FromPrimitive)]
#[repr(u16)]
pub enum Language {
    #[num_enum(default)]
    Invalid = 0,
    Japanese = 1,
    English = 2,
    French = 3,
    Italian = 4,
    German = 5,
    Spanish = 7,
    Korean = 8,
    ChineseS = 9,
    ChineseT = 10,
}

impl ToString for Language {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}
