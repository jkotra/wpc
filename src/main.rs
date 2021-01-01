#[macro_use]
extern crate clap;
use std::fs;
use std::fs::File;
use std::path::Path;

// JSON read/write
use serde_json;
use serde_json::{json, Value};

use clap::{App};

#[allow(unused_imports)]
use crate::misc::{wait, is_linux_gnome_de, print_debug_msg, download_wallpapers, random_n};

mod misc;

#[cfg(target_os = "linux")]
#[path = "changer/linux/DE/gnome.rs"] mod gnome;

#[cfg(target_os = "linux")]
#[path = "changer/linux/DE/startup.rs"] mod startup;

#[cfg(target_os = "windows")]
#[path = "changer/windows/windows.rs"] mod windows;

#[path = "web/wallhaven_api.rs"] mod wallhaven;
#[path = "web/bing_wpod.rs"] mod bing;

//this struct will be used to store wallhaven credentials.
struct WhCreds {
        username: String,
        coll_id: i64,
        api_key: String
}

// this struct will be used to update images.
struct WpcUpdateParams{
    bing: bool,
    wallhaven: bool,
    local: bool,
    only: bool,
    debug: bool,
    maxage: i64,
    wallhaven_creds: WhCreds,
    savepath: String,
}

#[tokio::main]
async fn main() {

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if matches.is_present("background"){
        misc::run_in_background();
        std::process::exit(0);
    }

    let debug = matches.occurrences_of("debug") != 0;
    let mut time_since = std::time::Instant::now();
    let savepath = matches.value_of("directory").unwrap();
    let mut local_flag = matches.is_present("local");
    let bing_flag = matches.is_present("bing");
    let wallhaven_flag = matches.is_present("wallhaven");
    let mut user_interval = matches.value_of("interval").unwrap().parse::<u64>().unwrap();
    let mut user_update_interval = matches.value_of("update").unwrap().parse::<u64>().unwrap();


    if !local_flag && !bing_flag && !wallhaven_flag{
        local_flag = true;
    }


    //if only flag, disable local
    if matches.is_present("only"){
        local_flag = false;
    }


    if cfg!(linux){
        if !misc::is_linux_gnome_de() {
            println!("OS / Distro not supported!");
        }
        else {
            if debug {
                print_debug_msg("GNOME Distro detected!");
            }
        }
    }

    if matches.is_present("startup"){
        #[cfg(target_os = "linux")]
        startup::add_to_startup_gnome();

        #[cfg(target_os = "windows")]
        windows::add_to_startup_reg();
    }

    //only bing is the argument
    if bing_flag &&
        !wallhaven_flag &&
        !local_flag {

        // set interval and update interval to 24 hrs
        user_update_interval = 60 * 60 * 24;
        user_interval = 60 * 60 * 24;

        if debug { print_debug_msg("interval and update_interval set to 24hrs!") }
    }


    let mut whcreds: WhCreds = WhCreds { username: String::from("None"), coll_id: -1, api_key: String::from("None") };
    let mut wpc_dir = std::env::current_exe().unwrap();
    wpc_dir.pop(); //remove wpc.exe from dir.

    let wh_json_path = format!("{}/wallhaven.json", wpc_dir.to_str().unwrap());

        if wallhaven_flag {

            // check if file wallhaven.json exists in CWD.
            if !Path::new(&wh_json_path).exists() {

                // ask user for username and coll_id
                let mut wh_username = String::new();
                let mut wh_coll_id = String::new();
                let mut wh_api_key = String::new();

                println!("\nwallhaven.cc Username:");
                std::io::stdin().read_line(&mut wh_username).unwrap();

                println!("\nwallhaven.cc Collection ID:");
                std::io::stdin().read_line(&mut wh_coll_id).unwrap();

                println!("\nwallhaven.cc API key (not required for public collection) (Get API key from https://wallhaven.cc/settings/account):");
                std::io::stdin().read_line(&mut wh_api_key).unwrap();

                //remove \n \r
                wh_username = wh_username.replace("\n", "").replace("\r", "");
                wh_coll_id = wh_coll_id.replace("\n", "").replace("\r", "");
                wh_api_key = wh_api_key.replace("\n", "").replace("\r", "");

                //convert wh_coll_id to int64
                let wh_coll_id = wh_coll_id.parse::<i64>().unwrap();

                // save user input to json
                let creds = json!({"wh_username": &wh_username, "wh_coll_id": wh_coll_id, "wh_api_key": wh_api_key });

                let wh_json_file = match File::create("wallhaven.json"){
                    Ok(file) => file,
                    Err(why) => panic!("cannot create file: {:?}", why)
                };

                let res = serde_json::to_writer(&wh_json_file, &creds);

                if res.is_err() {
                    panic!("cannot write to wallhaven.json");
                }
            }


            // read wallhaven.json
            let f = match fs::read_to_string(&wh_json_path){
                Ok(f) => f,
                Err(why) => panic!("cannot read config: {:?}", why),
            };

            let wh_json: Value = serde_json::from_str(&f).unwrap();

            whcreds.username = wh_json["wh_username"].as_str().unwrap().to_string();
            whcreds.coll_id = wh_json["wh_coll_id"].as_i64().unwrap();
            whcreds.api_key = wh_json["wh_api_key"].as_str().unwrap().to_string();
        }


        let wpc_up: WpcUpdateParams = WpcUpdateParams {
            bing: bing_flag,
            wallhaven: wallhaven_flag,
            local: local_flag,
            only: matches.is_present("only"),
            debug: debug,
            wallhaven_creds: whcreds,
            maxage: matches.value_of("maxage").unwrap().parse::<i64>().unwrap(),
            savepath: savepath.to_string()
        };

    let wpc_up = Box::new(wpc_up);


    //inner func
    fn change_wallpaper(debug: bool, file_list: &Vec<String>) {

        //print random number to user if debug enabled.
        let rand_n = random_n(file_list.len());
        if debug { println!("[DEBUG] RNG Result: {} total: {}", rand_n, file_list.len()) }

        let wp = file_list.get(rand_n).unwrap();
        if debug { misc::print_debug_msg(wp) }

        // Set wallpaper
        #[cfg(target_os = "linux")]
            gnome::change_wallpaper_gnome(wp);


        #[cfg(target_os = "windows")]
            windows::set_wallpaper_win(wp);

    }

    //initial
    let mut file_list = update_files(wpc_up.as_ref()).await.unwrap();
    change_wallpaper(debug, &file_list);


        //infinite loop
        loop {
            if debug { println!("[DEBUG] Waiting interval({})", user_interval) }
            wait(user_interval);

            change_wallpaper(debug, &file_list);


            if debug { println!("[DEBUG] Update interval: {} elapsed: {}", user_update_interval, time_since.elapsed().as_secs()) }
            if time_since.elapsed().as_secs() >= user_update_interval{
                file_list = update_files(wpc_up.as_ref()).await.unwrap();
                time_since = std::time::Instant::now();
            }
        }

        //Ok(())

    }


async fn update_files(params: &WpcUpdateParams) -> Result<Vec<String>, std::io::Error> {

    let mut file_list = Vec::new();


    if params.local{
        for file in misc::update_file_list(&params.savepath, params.maxage){
            file_list.push(file);
        }
    }

    if params.bing{

        for file in download_wallpapers(get_bing(), &params.savepath, true).await{

            file_list.push(file);
        }

        if !params.wallhaven && params.only{
                return Ok(file_list);
        }

    }

    if params.wallhaven{

        for file in download_wallpapers(get_wallhaven(params.wallhaven_creds.coll_id, &params.wallhaven_creds.username, &params.wallhaven_creds.api_key), &params.savepath, false).await{
            file_list.push(file)
        }

    }

    if file_list.len() == 0 { panic!("No images found in {}", params.savepath) }

    if params.debug {
        println!("[DEBUG] Updated file_list: {:?}", file_list);
    }

    return Ok(file_list)
}


fn get_bing() -> Vec<String> {
    let bing = bing::get_wallpaper_of_the_day();
    let bing = "https://bing.com".to_string()
        + bing.unwrap()["images"][0]["url"].as_str().unwrap().replace("&pid=hp","").as_str();
    return vec![bing]
}

fn get_wallhaven(collid: i64, username: &str, api_key: &str) -> Vec<String> {
    
    let collection: serde_json::value::Value;

    if api_key.contains("None"){
        collection = wallhaven::wallhaven_getcoll(username, collid).unwrap();
    }else{
        collection = wallhaven::wallhaven_getcoll_api(username, collid, api_key).unwrap();
    }
    
    let mut coll_urls: Vec<String> = vec![];

    for x in collection["data"].as_array() {
        for y in x {
            coll_urls.push(y["path"].as_str().unwrap().to_string())
        }
    }
    return coll_urls
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn bing_test_is_jpg() {
        let bing_url = super::get_bing();
        let bing_url: Vec<&str> = bing_url[0].split("&rf=").collect();
        assert_eq!(bing_url.get(1).unwrap().ends_with("jpg"),true)
    }

    #[test]
    fn bing_test_is_downloadable() {
        let bing_url = super::get_bing();
        let res = super::misc::download(bing_url.get(0).unwrap(),"target/test.jpg");
            assert_eq!(res.is_ok(), true)
    }

    #[test]
    fn wallhaven_get_wallpapers() {
        let res = super::wallhaven::wallhaven_getcoll("th4n0s", 803855);
        assert_eq!(res.is_ok(),true)
    }

}