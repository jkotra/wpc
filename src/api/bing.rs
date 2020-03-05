use serde_json::{Value};

pub fn get_wallpaper_of_the_day() -> Result<serde_json::value::Value, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://www.bing.com/HPImageArchive.aspx?format=js&idx=0&n=1&mkt=en-US").expect("Unable to make GET request!")
        .text()?;
    let v: Value = serde_json::from_str(&resp)
        .expect("Cannot Decode JSON Data!");
    Ok(v)
}