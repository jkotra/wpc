use serde_json::{Value};
use std::collections::HashMap;



// https://wallhaven.cc/help/api#wallpapers
#[allow(dead_code)]
pub fn wallhaven_wallpaperinfo(apikey: &str, id: &str) -> Result<serde_json::value::Value, Box<dyn std::error::Error>> {
    let url = format!("https://wallhaven.cc/api/v1/w/{id}?apikey={apikey}", apikey=apikey, id=id);
    let resp = reqwest::blocking::get(&url).expect("Unable to make GET request!")
        .text()?;
    let v: Value = serde_json::from_str(&resp)
        .expect("Cannot Decode JSON Data!");

    Ok(v)
}

// https://wallhaven.cc/help/api#search
#[allow(dead_code)]
pub fn wallhaven_search(apikey: &str, query: HashMap<&str, &str>) -> Result<Value, Box<dyn std::error::Error>> {

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
#[allow(dead_code)]
pub fn wallhaven_taginfo(id: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let url = format!("https://wallhaven.cc/api/v1/tag/{id}", id=id);

    let resp = reqwest::blocking::get(&url).expect("Unable to make GET request!")
        .text()?;
    let v: Value = serde_json::from_str(&resp)
        .expect("Cannot Decode JSON Data!");

    Ok(v)
}

// https://wallhaven.cc/help/api#user-settings
#[allow(dead_code)]
pub fn wallhaven_usersettings(apikey: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let url = format!("https://wallhaven.cc/api/v1/settings?apikey={apikey}", apikey=apikey);

    let resp = reqwest::blocking::get(&url).expect("Unable to make GET request!")
        .text()?;
    let v: Value = serde_json::from_str(&resp)
        .expect("Cannot Decode JSON Data!");

    Ok(v)
}

// https://wallhaven.cc/help/api#user-settings
#[allow(dead_code)]
pub fn wallhaven_getid(apikey: &str) -> Result<serde_json::value::Value, Box<dyn std::error::Error>> {
    let url = format!("https://wallhaven.cc/api/v1/collections?apikey={apik}", apik=apikey);

    let resp = reqwest::blocking::get(&url).expect("Unable to make GET request!")
        .text()?;
    let v: Value = serde_json::from_str(&resp)
        .expect("Cannot Decode JSON Data!");

    Ok(v)
}


// https://wallhaven.cc/help/api#user-settings - 2
#[allow(dead_code)]
pub fn wallhaven_getcoll(username: &str, collid: i64) -> Result<serde_json::value::Value, Box<dyn std::error::Error>> {
    let url = format!("https://wallhaven.cc/api/v1/collections/{username}/{collid}", username=username, collid=collid);

    let resp = reqwest::blocking::get(&url).expect("Unable to make GET request!")
        .text()?;
    let v: Value = serde_json::from_str(&resp)
        .expect("Cannot Decode JSON Data!");

    Ok(v)
}

#[allow(dead_code)]
pub fn wallhaven_getcoll_api(username: &str, collid: i64, api_key: &str) -> Result<serde_json::value::Value, Box<dyn std::error::Error>> {
    let url = format!("https://wallhaven.cc/api/v1/collections/{username}/{collid}?apikey={apik}", username=username, collid=collid, apik=api_key);

    let resp = reqwest::blocking::get(&url).expect("Unable to make GET request!")
        .text()?;
    let v: Value = serde_json::from_str(&resp)
        .expect("Cannot Decode JSON Data!");

    Ok(v)
}