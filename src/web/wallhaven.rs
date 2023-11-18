use crate::misc;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;

// JSON read/write
use serde_json;

use log::debug;

#[path = "wallhaven_api.rs"]
mod wallhaven_api;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct WallHaven {
    pub username: String,
    pub coll_id: String,
    pub api_key: String,
}

impl WallHaven {
    pub async fn update(&self, savepath: &PathBuf, maxage: i64) -> Vec<String> {
        let wallpaper_links = self.get_collection().await;
        debug!("wallhaven collection links = {:?}", wallpaper_links);
        let mut files = misc::download_wallpapers(wallpaper_links, &savepath).await;
        if maxage != -1 {
            files = misc::maxage_filter(files.clone(), maxage);
        }
        debug!("files from wallhaven = {:?}", files);
        return files;
    }

    async fn get_collection(&self) -> Vec<String> {
        let collection: serde_json::value::Value;

        collection =
            wallhaven_api::wallhaven_getcoll_api(&self.username, &self.coll_id, &self.api_key)
                .await
                .unwrap();

        let mut coll_urls: Vec<String> = vec![];

        if let Some(arr) = collection["data"].as_array() {
            for link in arr {
                coll_urls.push(link["path"].as_str().unwrap().to_string())
            }
        }

        return coll_urls;
    }

    pub fn init(&mut self, savepath: PathBuf) {
        // check if file wallhaven.json exists in CWD.
        if !savepath.exists() {
            // ask user for username and coll_id
            let mut wh_username = String::new();
            let mut wh_coll_id = String::new();
            let mut wh_api_key = String::new();

            println!("👤 Username:");
            std::io::stdin().read_line(&mut wh_username).unwrap();
            println!("📟 Collection ID:");
            std::io::stdin().read_line(&mut wh_coll_id).unwrap();
            println!("🔑 API key\n(not required for public collection, just press ENTER)\n(Get API key from https://wallhaven.cc/settings/account):");
            std::io::stdin().read_line(&mut wh_api_key).unwrap();

            self.username = wh_username.trim().to_string();
            self.coll_id = wh_coll_id.trim().to_string();
            self.api_key = wh_api_key.trim().to_string();

            let mut writer = std::io::BufWriter::new(std::fs::File::create(savepath).unwrap());
            match serde_json::to_writer_pretty(&mut writer, self) {
                Ok(j) => j,
                Err(err) => panic!("error wring wallhaven.json: {}", err),
            }
        } else {
            //read from json file
            self.read_json(savepath.to_str().unwrap())
        }
    }

    fn read_json(&mut self, wallhaven_json_path: &str) {
        let str_data = std::fs::read_to_string(wallhaven_json_path).unwrap();

        let data: WallHaven = serde_json::from_str(&str_data).unwrap();

        self.username = data.username;
        self.coll_id = data.coll_id;
        self.api_key = data.api_key;
    }
}

#[cfg(test)]
mod wallhaven {

    #[tokio::test]
    async fn wh_wallpaper() {
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
        let wh_coll = super::wallhaven_api::wallhaven_getcoll_api("th4n0s", "803855", "").await;
        assert_eq!(wh_coll.is_err(), false);
    }
}
