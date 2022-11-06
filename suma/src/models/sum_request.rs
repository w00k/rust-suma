use serde::{ Deserialize, Serialize };

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SumReq {
    pub x: i32,
    pub y: i32
}