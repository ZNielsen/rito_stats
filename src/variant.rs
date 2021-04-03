use std::collections::HashMap;
use std::vec::Vec;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Variant {
    Int(i64),
    Str(String),
    VecVar(Vec<HashMap<String, Variant>>),
    Hash(HashMap<String, Variant>),
}
impl Into<String> for Variant {
    fn into(self) -> String {
        match self {
            Variant::Str(s) => s,
            _ => panic!("Not a string: {:?}", self),
        }
    }
}
impl Into<i64> for Variant {
    fn into(self) -> i64 {
        match self {
            Variant::Int(i) => i,
            _ => panic!("Not an int: {:?}", self),
        }
    }
}
impl Into<Vec<HashMap<String, Variant>>> for Variant {
    fn into(self) -> Vec<HashMap<String, Variant>> {
        match self {
            Variant::VecVar(v) => v,
            _ => panic!("Not a Vec: {:?}", self),
        }
    }
}
impl Into<HashMap<String, Variant>> for Variant {
    fn into(self) -> HashMap<String, Variant> {
        match self {
            Variant::Hash(h) => h,
            _ => panic!("Not a Hash: {:?}", self),
        }
    }
}
