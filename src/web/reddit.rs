use std::path::PathBuf;

use crate::misc;
use crate::settings::WPCSettings;
use async_trait::async_trait;
use clap::ValueEnum;
use log::debug;
use reqwest;
use serde_json;
use serde_json::Value;

use crate::web::WPCPlugin;

use super::PluginDetails;

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
}

#[async_trait]
impl WPCPlugin for Reddit {
    fn details(&self) -> super::PluginDetails {
        return PluginDetails {
            name: "Reddit".to_string(),
            version: "1.0".to_string(),
            author: "Jagadeesh Kotra <jagadeesh@stdin.top>".to_string(),
        };
    }
    async fn init(
        &mut self,
        _config_file: Option<PathBuf>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    async fn init_from_settings(
        &mut self,
        app_settings: WPCSettings,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.subreddit = app_settings.reddit_options.reddit;
        self.n = app_settings.reddit_options.reddit_n;
        self.cat = app_settings.reddit_options.reddit_sort;
        Ok(())
    }
    async fn update(&self, savepath: &PathBuf, maxage: i64) -> Vec<String> {
        let urls = get_pictures_from_subreddit(&self.subreddit, self.n, self.cat).await;
        debug!("URLs from reddit = {:?}", urls);
        let files = misc::download_wallpapers(urls, savepath).await;
        let files = misc::maxage_filter(files, maxage);

        let processed_files = files.clone();

        debug!("files from reddit = {:?}", processed_files);
        return processed_files;
    }
}

#[cfg(test)]
mod reddit {

    use crate::web::reddit::RedditSort;

    #[tokio::test]
    async fn reddit_test_get_image_urls_from_subreddit() {
        if std::env::var("GCP_CLOUD_BUILD").is_ok() {
            return;
        };
        let urls = super::get_pictures_from_subreddit("art", 5, RedditSort::Hot).await;
        assert_eq!(urls.len(), 5);
    }
}
