use std::path::PathBuf;

use crate::misc;
use clap::ValueEnum;
use image;
use image::GenericImageView;
use log::debug;
use reqwest;
use serde_json;
use serde_json::Value;

async fn get_pictures_from_subreddit(subreddit: &str, n: i64, cat: RedditSort) -> Vec<String> {
    let mut url = String::from("https://reddit.com/r/") + subreddit;

    match cat {
        RedditSort::Hot => url += "/hot/",
        RedditSort::New => url += "/new/",
        RedditSort::Top => url += "/top/",
        RedditSort::Rising => url += "/rising/",
        _ => url += "/hot/", //default
    }

    url += ".json";

    let mut file_vec: Vec<String> = vec![];

    let client = reqwest::Client::builder()
        .user_agent("wpc")
        .build()
        .unwrap();
    let resp = client.get(&url).send().await.unwrap().text().await.unwrap();

    let data: Value = serde_json::from_str(&resp).unwrap();

    let mut link_count = 0;

    for thread in data["data"]["children"].as_array().unwrap() {
        let url = match thread["data"]["url_overridden_by_dest"].as_str() {
            Some(link) => link,
            None => continue,
        };

        if url.contains("png") || url.contains("jpg") || url.contains("jpeg") {
            file_vec.push(String::from(url));
            link_count += 1;
        }

        if link_count >= n {
            break;
        }
    }

    return file_vec;
}

#[derive(Debug, Default, Clone, Copy, ValueEnum)]
pub enum RedditSort {
    #[default]
    Hot,
    Popular,
    New,
    Top,
    Rising,
}

#[derive(Default, Debug)]
pub struct Reddit {
    pub subreddit: String,
    pub cat: RedditSort,
    pub n: i64,

    pub min_height: u32,
    pub min_width: u32,
}

impl Reddit {
    pub async fn update(&self, savepath: &PathBuf, maxage: i64) -> Vec<String> {
        let urls = get_pictures_from_subreddit(&self.subreddit, self.n, self.cat).await;
        debug!("URLs from reddit = {:?}", urls);
        let files = misc::download_wallpapers(urls, savepath).await;
        let files = misc::maxage_filter(files, maxage);

        let mut processed_files = vec![];

        if self.min_height > 0 && self.min_width > 0 {
            for i in 0..files.len() {
                let img = image::open(&files[i]).unwrap();
                let (width, height) = img.dimensions();

                if (width >= self.min_width) || (height >= self.min_height) {
                    processed_files.push(String::from(&files[i]));
                } else {
                }
            }
        } else {
            // user did not chose to filter w, h
            processed_files = files.clone();
        }

        debug!("files from reddit = {:?}", processed_files);
        return processed_files;
    }
}

#[cfg(test)]
mod reddit {

    use crate::web::reddit::RedditSort;

    #[tokio::test]
    async fn reddit_test_get_image_urls_from_subreddit() {
        let urls = super::get_pictures_from_subreddit("art", 5, RedditSort::Hot).await;
        assert_eq!(urls.len(), 5);
    }
}
