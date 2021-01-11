use serde_json::Value;
use reqwest;


pub async fn get_bing_wpod() -> Vec<String> {

    let resp = reqwest::get(
        "https://www.bing.com/HPImageArchive.aspx?format=js&idx=0&n=1&mkt=en-US",
    ).await.unwrap().text().await.unwrap();

    let v: Value = serde_json::from_str(&resp).expect("Cannot Decode JSON Data!");

    let bing_url = "https://bing.com".to_string()
    + v["images"][0]["url"].as_str().unwrap().replace("&pid=hp","").as_str();

    let url: Vec<&str> = bing_url.split("&rf=").collect();
    let url = url[0];

    let mut url_vec: Vec<String> = Vec::new();
    url_vec.push(url.to_string());

    return url_vec;
}