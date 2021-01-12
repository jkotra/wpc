#[macro_use]
extern crate clap;
use std::fs::File;
use std::path::Path;

// JSON read/write
use serde_json;
use serde_json::{json, Value};

use clap::App;

#[allow(unused_imports)]
use crate::misc::{download_wallpapers, is_linux_gnome_de, random_n, wait, WPCDebug};

mod misc;

#[cfg(target_os = "linux")]
#[path = "changer/linux/DE/gnome.rs"]
mod gnome;

#[cfg(target_os = "linux")]
#[path = "changer/linux/DE/startup.rs"]
mod startup;

#[cfg(target_os = "windows")]
#[path = "changer/windows/windows.rs"]
mod windows;

#[path = "web/bing_wpod.rs"]
mod bing;


#[path = "web/wallhaven_api.rs"]
mod wallhaven;

#[derive(Default)]
struct WallHaven {
    username: String,
    coll_id: i64,
    api_key: String,
}

impl WallHaven {
    async fn update(&self, savepath: &str, maxage: i64, wpc_debug: &WPCDebug) -> Vec<String> {

        let wallpaper_links = self.get_collection(wpc_debug);
        let mut files = download_wallpapers(wallpaper_links, &savepath, wpc_debug).await; 
        if maxage != -1 {
            files = misc::maxage_filter(files.clone(), maxage, &wpc_debug); 
        }
        return files;
    }

    fn get_collection(&self, wpc_debug: &WPCDebug) -> Vec<String> {
        let collection: serde_json::value::Value;

        loop {
            collection = match wallhaven::wallhaven_getcoll_api(&self.username, self.coll_id, &self.api_key) {
                Ok(c) => c,
                Err(c) => {
                    println!(":{:?}", c);
                    wait(5);
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

        wpc_debug.debug(
            format!("links parsed from collection ID {} = {} = {:?}", self.coll_id, coll_urls.len(), coll_urls)
        );

        return coll_urls;
    }

}

#[tokio::main]
async fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let is_debug = matches.occurrences_of("debug") != 0;
    let main_debug = WPCDebug { is_debug: is_debug };

    let mut time_since = std::time::Instant::now();
    let maxage = matches.value_of("maxage").unwrap().parse::<i64>().unwrap();
    let savepath: String;
    let _savepath = matches.value_of("directory").unwrap();
    if _savepath.eq_ignore_ascii_case(".") {
        savepath = String::from(std::env::current_dir().unwrap().to_str().unwrap());
    } else {
        savepath = String::from(_savepath)
    }
    let savepath = &savepath;

    let mut local_flag = true;
    let bing_flag = matches.is_present("bing");
    let mut wallhaven_flag = matches.is_present("wallhaven");
    let mut user_interval = matches
        .value_of("interval")
        .unwrap()
        .parse::<u64>()
        .unwrap();
    let mut user_update_interval = matches.value_of("update").unwrap().parse::<u64>().unwrap();
    

    //defaults to local if there are no flags.
    if bing_flag || wallhaven_flag {
        local_flag = false;
    }

    if bing_flag{
        wallhaven_flag = false;
        local_flag = false;

        user_update_interval = 60 * 60 * 24;
        user_interval = 60 * 60 * 24;
    }

    main_debug.debug(
        format!("local={}, wallhaven={}, bing={}", local_flag, wallhaven_flag, bing_flag)
    );


    if cfg!(linux) {
        if !misc::is_linux_gnome_de() {
            panic!("DE not supported!");
        }
    }

    if matches.is_present("startup") {
        #[cfg(target_os = "linux")]
        startup::add_to_startup_gnome();

        #[cfg(target_os = "windows")]
        windows::add_to_startup_reg();
    }

    if matches.is_present("background") {
        misc::run_in_background(&main_debug);
        std::process::exit(0);
    }


    let mut wallhaven_cc = WallHaven {
        ..Default::default()
    };

    if wallhaven_flag {
        wallhaven_init("wallhaven.json");
        wallhaven_cc = wallhaven_read_json();
    }

    let mut candidates: Vec<String> = Vec::new();

    while candidates.len() == 0{
        if wallhaven_flag{
            candidates = wallhaven_cc.update(savepath, maxage, &main_debug).await
        }
        else if bing_flag{
            candidates = bing::get_bing_wpod().await;
            candidates = download_wallpapers(candidates.clone(), savepath, &main_debug).await;
        }
        else if local_flag{
            candidates = update_local_files(savepath, maxage, &main_debug);
        }
        else{
            panic!("Unknown arg. configuration!")
        }
    }
    
    // main loops, deals with waiting interval and updates.
    loop {

        //change wallpaper 
        change_wallpaper_random(&candidates, &main_debug);

        wait(user_interval);
        main_debug.debug(
            format!("u = {} elapsed = {}", user_update_interval, time_since.elapsed().as_secs())
        );

        if time_since.elapsed().as_secs() >= user_update_interval {
            if local_flag {
                candidates = update_local_files(savepath,
                                                maxage, &main_debug);
            }

            if wallhaven_flag {
                candidates = wallhaven_cc.update(savepath, maxage, &main_debug).await;
            }

            time_since = std::time::Instant::now();
        }
    }
}

fn change_wallpaper_random(file_list: &Vec<String>, wpc_debug: &WPCDebug) {
    //print random number to user if debug enabled.
    let rand_n = random_n(file_list.len());
    let wp = file_list.get(rand_n).unwrap();
    
    wpc_debug.debug(format!("Total = {} rand_n = {}", file_list.len(), rand_n));
    wpc_debug.info(String::from(wp));


    #[cfg(target_os = "linux")]
    gnome::change_wallpaper_gnome(wp);

    #[cfg(target_os = "windows")]
    windows::set_wallpaper_win(wp);
}

fn update_local_files(savepath: &str, max_age: i64, wpc_debug: &WPCDebug) -> Vec<String> {
    let mut file_list: Vec<String> = Vec::new();

    for file in misc::update_file_list(savepath, max_age, wpc_debug) {
        file_list.push(file);
    }

    return file_list;
}

fn wallhaven_init(wallhaven_json_path: &str) {
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

fn wallhaven_read_json() -> WallHaven {
    let wh_json: Value = serde_json::from_str(std::fs::read_to_string("wallhaven.json").unwrap().as_ref()).unwrap();
    let mut wh = WallHaven {
        ..Default::default()
    };

    wh.username = wh_json["wh_username"].as_str().unwrap().to_string();
    wh.coll_id = wh_json["wh_coll_id"].as_i64().unwrap();
    wh.api_key = wh_json["wh_api_key"].as_str().unwrap().to_string();

    return wh;
}


#[cfg(test)]
mod main {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
