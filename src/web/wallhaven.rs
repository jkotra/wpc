use crate::misc;
use crate::settings::WPCSettings;
use async_trait::async_trait;
use serde::Deserialize;
use serde::Serialize;
use std::error::Error;
use std::path::PathBuf;

use serde_json;

use log::debug;

use super::PluginDetails;
use super::WPCPlugin;

#[path = "wallhaven_api.rs"]
mod wallhaven_api;

fn init_wizard(config_file_path: PathBuf) -> WallHaven {
    let mut wh_username = String::new();
    let mut wh_collection_id = String::new();
    let mut wh_api_key = String::new();

    println!("👤 Username:");
    std::io::stdin().read_line(&mut wh_username).unwrap();
    println!("📟 Collection ID:");
    std::io::stdin().read_line(&mut wh_collection_id).unwrap();
    println!("🔑 API key\n(not required for public collection, just press ENTER)\n(Get API key from https://wallhaven.cc/settings/account):");
    std::io::stdin().read_line(&mut wh_api_key).unwrap();

    let data = WallHaven {
        username: wh_username,
        collection_id: wh_collection_id,
        api_key: wh_api_key,
    };

    let mut writer = std::io::BufWriter::new(std::fs::File::create(config_file_path).unwrap());
    match serde_json::to_writer_pretty(&mut writer, &data) {
        Ok(j) => j,
        Err(err) => panic!("error writing config file: {}", err),
    }
    return data;
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct WallHaven {
    pub username: String,
    pub collection_id: String,
    pub api_key: String,
}

#[async_trait]
impl WPCPlugin for WallHaven {
    async fn init(&mut self, config_file: Option<PathBuf>) -> Result<(), Box<dyn Error>> {
        let data = match config_file.clone().unwrap().exists() {
            true => {
                let str_data = std::fs::read_to_string(config_file.unwrap()).unwrap();
                serde_json::from_str(&str_data).unwrap()
            }
            false => init_wizard(config_file.unwrap()),
        };
        self.username = data.username;
        self.collection_id = data.collection_id;
        self.api_key = data.api_key;
        Ok(())
    }
    async fn init_from_settings(&mut self, _settings: WPCSettings) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
    fn details(&self) -> super::PluginDetails {
        return PluginDetails {
            name: "WallHaven".to_string(),
            version: "1.0".to_string(),
            author: "Jagadeesh Kotra <jagadeesh@stdin.top>".to_string(),
        };
    }

    async fn update(&self, savepath: &PathBuf, maxage: i64) -> Vec<String> {
        let collection: serde_json::value::Value;

        collection = wallhaven_api::wallhaven_getcoll_api(
            &self.username,
            &self.collection_id,
            &self.api_key,
        )
        .await
        .unwrap();

        let mut collection_urls: Vec<String> = vec![];

        if let Some(arr) = collection["data"].as_array() {
            for link in arr {
                collection_urls.push(link["path"].as_str().unwrap().to_string())
            }
        }

        debug!("wallhaven collection links = {:?}", collection_urls);
        let mut files = misc::download_wallpapers(collection_urls, &savepath).await;
        if maxage != -1 {
            files = misc::maxage_filter(files.clone(), maxage);
        }
        debug!("files from wallhaven = {:?}", files);
        return files;
    }
}

#[cfg(test)]
mod wallhaven {

    #[tokio::test]
    async fn wh_wallpaper() {
        if std::env::var("GCP_CLOUD_BUILD").is_ok() {
            return;
        };
        let wp_info = super::wallhaven_api::wallhaven_wallpaperinfo("", "q6jvjl")
            .await
            .unwrap();
        println!("{:?}", wp_info);
        let x = format!("{}", wp_info["data"]["dimension_x"]);
        let y = format!("{}", wp_info["data"]["dimension_y"]);
        assert_eq!(x, "1920");
        assert_eq!(y, "1080");
    }
    #[tokio::test]
    async fn wh_getcoll() {
        if std::env::var("GCP_CLOUD_BUILD").is_ok() {
            return;
        };
        let wh_coll = super::wallhaven_api::wallhaven_getcoll_api("th4n0s", "803855", "").await;
        assert_eq!(wh_coll.is_err(), false);
    }
}
