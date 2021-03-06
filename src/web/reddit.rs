use reqwest;
use serde_json;
use serde_json::Value;
use image;
use image::{ GenericImageView};
use crate::misc;
use misc::WPCDebug;



async fn get_pictures_from_subreddit(subreddit: &str, n: i64, cat: &str) -> Vec<String> {


    let mut url = String::from("https://reddit.com/r/") + subreddit;

    match cat {
        "hot" => { url += "/hot/" },
        "new" => {url += "/new/" },
        "top" => {url += "/top/"},
        "rising" => {url += "/rising/" },
        _ => { url += "/hot/" }, //default
    }

    url += ".json";

    let mut file_vec: Vec<String> = vec![];

    let resp = reqwest::get(&url).await.unwrap().text().await.unwrap();

    let data: Value = serde_json::from_str(&resp).unwrap();

    let mut link_count = 0;

    for thread in data["data"]["children"].as_array().unwrap(){

        let url = match thread["data"]["url_overridden_by_dest"].as_str(){
            Some(link) => link,
            None => continue
        };
        
        if url.contains("png") || url.contains("jpg") || url.contains("jpeg") {
            file_vec.push(String::from(url));
            link_count += 1;
        }

        if link_count >= n{
            break;
        }

    }

    return file_vec;
}

pub struct Reddit{
    pub subreddit: String,
    pub cat: String,
    pub n: i64,

    pub min_height: u32,
    pub min_width: u32,
}

impl Reddit {
    pub async fn update(&self, savepath: &str,maxage: i64, wpc_debug: &WPCDebug) -> Vec<String> {
        
        let urls = get_pictures_from_subreddit(&self.subreddit, self.n, &self.cat).await;
        let files = misc::download_wallpapers(urls, savepath, wpc_debug).await;
        let files = misc::maxage_filter(files, maxage, wpc_debug);

        let mut processed_files = vec![];

        if self.min_height > 0 && self.min_width > 0{

            for i in 0..files.len(){
                let img = image::open(&files[i]).unwrap();
                let (width,height) = img.dimensions();

                if (width >= self.min_width) || (height >= self.min_height){
                    processed_files.push(String::from(&files[i]));
                }
                else{
                    wpc_debug.debug(format!("Reddit image Skipped: {} dim = [{}, {}], min-required = [{}, {}]",&files[i], width, height, self.min_width, self.min_height));
                    //processed_files.push(String::from(&files[i]));
                }
            }

        }
        else{
            // user did not chose to filter w, h
            processed_files = files.clone();
        }
        

        wpc_debug.debug(format!("Files from reddit = {} = {:?}", processed_files.len(), processed_files));
        return processed_files;
    }
}

#[cfg(test)]
mod reddit {

    #[tokio::test]
    async fn reddit_test_get_image_urls_from_subreddit(){
        let urls = super::get_pictures_from_subreddit("wallpaper", 5, "hot").await;
        assert_eq!(urls.len(), 5);
    }

}