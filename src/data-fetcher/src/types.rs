use std::fmt::Debug;

use serde::Deserialize;

pub trait SportMonks: for<'a> Deserialize<'a> + Debug {}

#[derive(Debug, Deserialize)]
pub enum TeamsType {
    #[serde(rename = "domestic")] 
    Domestic
}

#[derive(Deserialize, Debug)]
pub struct Teams {
    pub id: u32,
    pub name: String,
    pub short_code: Option<String>,
    #[serde(rename = "type")] 
    pub teams_type: TeamsType,
    pub image_path: String,
    pub founded: u32,
}

impl SportMonks for Teams {}


#[derive(Deserialize, Debug)]
pub struct Response<T> {
    pub data: T
    /*
    pagination: String,
    subscription: String,
    rate_limit: String,
    timezone: String
    */
}