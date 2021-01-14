use crate::misc; 
use misc::WPCDebug;

use std::fs::File;
use std::path::Path;

// JSON read/write
use serde_json;
use serde_json::{json, Value};

mod wallhaven_api;

#[derive(Default)]
pub struct WallHaven {
    pub username: String,
    pub coll_id: i64,
    pub api_key: String,
}

impl WallHaven {
    pub async fn update(&self, savepath: &str, maxage: i64, wpc_debug: &WPCDebug) -> Vec<String> {
        let wallpaper_links = self.get_collection(wpc_debug);
        let mut files = misc::download_wallpapers(wallpaper_links, &savepath, &wpc_debug).await;
        if maxage != -1 {
            files = misc::maxage_filter(files.clone(), maxage, wpc_debug);
        }
        return files;
    }

    fn get_collection(&self, wpc_debug: &WPCDebug) -> Vec<String> {
        let collection: serde_json::value::Value;

        loop {
            collection = match wallhaven_api::wallhaven_getcoll_api(&self.username, self.coll_id, &self.api_key) {
                Ok(c) => c,
                Err(c) => {
                    println!(":{:?}", c);
                    misc::wait(5);
                    continue;
                }
            };
            break;
        }

        let mut coll_urls: Vec<String> = vec![];

        for x in collection["data"].as_array() {
            for y in x {
                coll_urls.push(y["path"].as_str().unwrap().to_string())
            }
        }

        wpc_debug.debug(format!(
            "links parsed from collection ID {} = {} = {:?}",
            self.coll_id,
            coll_urls.len(),
            coll_urls
        ));

        return coll_urls;
    }

    pub fn init(&self, wallhaven_json_path: &str) {
        // check if file wallhaven.json exists in CWD.
        if !Path::new(wallhaven_json_path).exists() {
            // ask user for username and coll_id
            let mut wh_username = String::new();
            let mut wh_coll_id = String::new();
            let mut wh_api_key = String::new();
            println!("wallhaven.cc Username:");
            std::io::stdin().read_line(&mut wh_username).unwrap();
            println!("wallhaven.cc Collection ID:");
            std::io::stdin().read_line(&mut wh_coll_id).unwrap();
            println!("\nwallhaven.cc API key (not required for public collection) (Get API key from https://wallhaven.cc/settings/account):");
            std::io::stdin().read_line(&mut wh_api_key).unwrap();
            wh_username = wh_username.replace("\n", "").replace("\r", "");
            wh_coll_id = wh_coll_id.replace("\n", "").replace("\r", "");
            wh_api_key = wh_api_key.replace("\n", "").replace("\r", "");
            //convert wh_coll_id to int64
            let wh_coll_id = wh_coll_id.parse::<i64>().unwrap();
            // save user input to json
            let creds = json!({"wh_username": &wh_username, "wh_coll_id": wh_coll_id, "wh_api_key": wh_api_key });
            let wh_json_file = match File::create("wallhaven.json") {
                Ok(file) => file,
                Err(why) => panic!("cannot create file: {:?}", why),
            };
            let res = serde_json::to_writer(&wh_json_file, &creds);
            if res.is_err() {
                panic!("cannot write to wallhaven.json")
            }
        }
    }

    pub fn read_json(&self, wallhaven_json_path: &str) -> WallHaven {

        let mut json_file = std::path::PathBuf::from(wallhaven_json_path);
        json_file.push("wallhaven.json");

        let wh_json: Value =
            serde_json::from_str(std::fs::read_to_string(json_file.to_str().unwrap()).unwrap().as_ref())
                .unwrap();

        let mut wh = WallHaven {
            ..Default::default()
        };
        wh.username = wh_json["wh_username"].as_str().unwrap().to_string();
        wh.coll_id = wh_json["wh_coll_id"].as_i64().unwrap();
        wh.api_key = wh_json["wh_api_key"].as_str().unwrap().to_string();

        return wh;
    }
}

#[cfg(test)]
mod wallhaven {

    #[test]
    fn wh_wallpaper() {
        let wp_info = super::wallhaven_api::wallhaven_wallpaperinfo("", "q6jvjl").unwrap();
        println!("{:?}", wp_info);
        let x = format!("{}", wp_info["data"]["dimension_x"]);
        let y = format!("{}", wp_info["data"]["dimension_y"]);
        assert_eq!(x, "1920");
        assert_eq!(y, "1080");
    }
    #[test]
    fn wh_getcoll() {
        let wh_coll = super::wallhaven_api::wallhaven_getcoll_api("th4n0s", 803855, "");
        assert_eq!(wh_coll.is_err(), false);
    }
}
