use serde_json::{Value};
use std::collections::HashMap;


// https://wallhaven.cc/help/api#wallpapers
pub fn wallheaven_wallpaperinfo(apikey: &str, id: &str) -> Result<serde_json::value::Value, Box<dyn std::error::Error>> {
    let url = format!("https://wallhaven.cc/api/v1/w/{id}?apikey={apikey}", apikey=apikey, id=id);
    let resp = reqwest::blocking::get(&url).expect("Unable to make GET request!")
        .text()?;
    let v: Value = serde_json::from_str(&resp)
        .expect("Cannot Decode JSON Data!");

    Ok(v)
}

// https://wallhaven.cc/help/api#search
pub fn wallheaven_search(apikey: &str, query: HashMap<&str, &str>) -> Result<Value, Box<dyn std::error::Error>> {

    let mut base = format!("https://wallhaven.cc/api/v1/search?apikey={apikey}&", apikey=apikey);
    for (k,v) in query{
        base = base + &format!("{key}={val}", key=k,val=v);
    }

    let resp = reqwest::blocking::get(&base).expect("Unable to make GET request!")
        .text()?;
    let v: Value = serde_json::from_str(&resp)
        .expect("Cannot Decode JSON Data!");

    Ok(v)
}

// https://wallhaven.cc/help/api#tags
pub fn wallheaven_taginfo(id: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let url = format!("https://wallhaven.cc/api/v1/tag/{id}", id=id);

    let resp = reqwest::blocking::get(&url).expect("Unable to make GET request!")
        .text()?;
    let v: Value = serde_json::from_str(&resp)
        .expect("Cannot Decode JSON Data!");

    Ok(v)
}

// https://wallhaven.cc/help/api#user-settings
pub fn wallheaven_usersettings(apikey: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let url = format!("https://wallhaven.cc/api/v1/settings?apikey={apikey}", apikey=apikey);

    let resp = reqwest::blocking::get(&url).expect("Unable to make GET request!")
        .text()?;
    let v: Value = serde_json::from_str(&resp)
        .expect("Cannot Decode JSON Data!");

    Ok(v)
}

// https://wallhaven.cc/help/api#user-settings
pub fn wallheaven_getid(apikey: &str) -> Result<serde_json::value::Value, Box<dyn std::error::Error>> {
    let url = format!("https://wallhaven.cc/api/v1/collections?apikey={apik}", apik=apikey);

    let resp = reqwest::blocking::get(&url).expect("Unable to make GET request!")
        .text()?;
    let v: Value = serde_json::from_str(&resp)
        .expect("Cannot Decode JSON Data!");

    Ok(v)
}


// https://wallhaven.cc/help/api#user-settings - 2
pub fn wallheaven_getcoll(username: &str, collid: i64) -> Result<serde_json::value::Value, Box<dyn std::error::Error>> {
    let url = format!("https://wallhaven.cc/api/v1/collections/{username}/{collid}", username=username, collid=collid);

    let resp = reqwest::blocking::get(&url).expect("Unable to make GET request!")
        .text()?;
    let v: Value = serde_json::from_str(&resp)
        .expect("Cannot Decode JSON Data!");

    Ok(v)
}