use serde_json::{Value};

#[allow(dead_code)]
fn get_wallpaper_of_the_day() -> Result<serde_json::value::Value, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://www.bing.com/HPImageArchive.aspx?format=js&idx=0&n=1&mkt=en-US").expect("Unable to make GET request!")
        .text()?;
    let v: Value = serde_json::from_str(&resp)
        .expect("Cannot Decode JSON Data!");
    Ok(v)
}

#[allow(dead_code)]
pub fn get_bing() -> Vec<String> {
    let bing = get_wallpaper_of_the_day();
    let bing = "https://bing.com".to_string()
        + bing.unwrap()["images"][0]["url"].as_str().unwrap().replace("&pid=hp","").as_str();
    return vec![bing]
}