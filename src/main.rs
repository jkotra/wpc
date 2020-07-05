#[macro_use]
extern crate clap;
use std::fs;
use std::fs::File;
use std::path::Path;

use serde_json;
use serde_json::{json, Value};

use clap::{App, ArgMatches};
use std::time::{Duration, Instant};
use crate::misc::{update_file_list, print_debug_msg, add_to_startup_gnome};

#[path = "api/wallheaven.rs"] mod wallheaven;
#[path = "api/bing.rs"] mod bing;

#[cfg(target_os = "linux")]
#[path = "api/distro/kde.rs"] mod kde;

#[cfg(target_os = "linux")]
#[path = "api/distro/gnome.rs"] mod gnome;

#[path = "api/distro/lib.rs"] mod lib;

#[cfg(target_os = "windows")]
#[path = "api/os/windows.rs"] mod windows;


mod misc;

//this stuct will be used to store wallheaven credentials.
struct WhCreds {
        username: String,
        coll_id: i64,
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let debug = matches.occurrences_of("debug") != 0;
    let mut time = std::time::Instant::now();

    let is_linux = cfg!(linux);
    let is_windows = cfg!(windows);

    if (matches.is_present("startup")){
        println!("Adding WPC to startup...");
        add_to_startup_gnome(matches.value_of("directory").unwrap().to_string(),
                            matches.value_of("interval").unwrap().parse::<i32>().unwrap(),
                             matches.value_of("update").unwrap().parse::<i32>().unwrap());
    }

    let mut whcreds: WhCreds = WhCreds { username: String::from("None"), coll_id: 0 };

    #[cfg(target_os = "linux")]
    fn get_linux_distro() ->  String {
            if misc::is_linux_gnome_de() { return "gnome".to_string() }
            else if misc::is_linux_kde_de() { return "kde".to_string() }
            else { return "Not Supported".to_string() }
    }
    #[cfg(target_os = "linux")]
    let linux_distro = get_linux_distro();

    #[cfg(target_os = "linux")]{
    if debug { print_debug_msg( linux_distro.as_str() ) }
    if linux_distro == "Not Supported" { panic!("Distro not supported!") }
    }


    // let mut wallpaper_manifest: Vec<String> = vec![];
    //let mut file_manifest: Vec<String> = vec![];

    let savepath = matches.value_of("directory").unwrap();

    // check flags
    let bing_flag = matches.is_present("bing");

    let wallheaven_flag = matches.is_present("wallheaven");
    
    if wallheaven_flag {

        // check if file wallheaven.json exists in cwd.
        if !Path::new("wallheaven.json").exists() {

            // create file

            // ask user input
            let mut wh_username = String::new();
            let mut wh_coll_id = String::new();


            println!("Wallheaven Username:");
            std::io::stdin().read_line(&mut wh_username).unwrap();

            println!("\nWallheaven Collection ID:");
            std::io::stdin().read_line(&mut wh_coll_id).unwrap();

            //remove \n \r
            wh_username = wh_username.replace("\n", "");
            wh_username = wh_username.replace("\r", "");

            wh_coll_id = wh_coll_id.replace("\n", "");
            wh_coll_id = wh_coll_id.replace("\r", "");

            //convert wh_coll_id to int64
            let wh_coll_id = wh_coll_id.parse::<i64>().unwrap();

            // save user input to json
            let creds = json!({"wh_username": &wh_username, "wh_coll_id": wh_coll_id });
            let mut wh_json_file = File::create("wallheaven.json").expect("failed to create file");
            serde_json::to_writer(&wh_json_file, &creds);
        }


        // read wallheaven.json
        let f = fs::read_to_string("wallheaven.json");
        let wh_json_buf: Value = serde_json::from_str(&f.unwrap()).unwrap();

        whcreds.username = wh_json_buf["wh_username"].as_str().unwrap().to_string();
        whcreds.coll_id = wh_json_buf["wh_coll_id"].as_i64().unwrap()
}

    let local_flag = matches.is_present("local");

    let file_manifest = update(bing_flag,
                               wallheaven_flag,
                               &whcreds,
                               local_flag,
                               matches.clone(),
                               savepath);

    // last check
    if file_manifest.len() == 0{
        panic!("No files added to file_manifest!")
    }

    loop {
        let mut user_interval = matches.value_of("interval").unwrap().parse::<u64>().unwrap();
        let mut user_update_interval = matches.value_of("update").unwrap().parse::<u64>().unwrap();

        //only bing is the argument
        if matches.is_present("bing") &&
            !matches.is_present("wallheaven") &&
            !matches.is_present("local") {
            // set interval and update interval to 24 hrs
            user_update_interval = 60*60*24;
            user_interval = 60*60*24;
        }

        let rand_n = misc::random_n(file_manifest.len());

        //print random number to user if debug enabled.
        if (debug) { println!("Random number: {} total: {}", rand_n, file_manifest.len()) }
        let wp = file_manifest.get(rand_n).unwrap();
        if debug { print_debug_msg(wp) }

        // Set wallpaper according to OS and / Distro
        #[cfg(target_os = "linux")]{
            let is_de = (linux_distro == "gnome");
            if is_de { gnome::change_wallpaper_gnome(wp); }

            // KDE / Plasma
            let is_de = (linux_distro == "kde");
            if is_de == true { kde::set(wp) }
        }

        #[cfg(target_os = "windows")]
        windows::set_wallpaper_win(wp);
        
        if time.elapsed().as_secs() > user_update_interval {

            if debug {print_debug_msg("Updating Images..") }

            // update stuff here
            let file_manifest = update(bing_flag,wallheaven_flag, &whcreds, local_flag,matches.to_owned(), savepath);

            //print file_manifest to terminal if debug enabled.
            if debug {
                println!("{:?}", file_manifest)
            }

            time = Instant::now();
        }
        misc::wait(user_interval);
    }

}


fn get_bing() -> Vec<String> {
    let bing = bing::get_wallpaper_of_the_day();
    let bing = "https://bing.com".to_string()
        + bing.unwrap()["images"][0]["url"].as_str().unwrap().replace("&pid=hp","").as_str();
    return vec![bing]
}

fn get_wallheaven(collid: i64, username: &str) -> Vec<String> {
    let collection = wallheaven::wallheaven_getcoll(username, collid);
    let mut coll_urls: Vec<String> = vec![];

    for x in collection.unwrap()["data"].as_array() {
        for y in x {
            coll_urls.push(y["path"].as_str().unwrap().to_string())
        }
    }
    return coll_urls
}

fn update(bing: bool, wallheaven: bool, wallheaven_creds: &WhCreds, local: bool,matches: ArgMatches, savepath: &str) -> Vec<String>{
    let mut file_manifest: Vec<String> = vec![];
    let mut fileman: Vec<String> = vec![];

    if bing{

        let bing_url = get_bing();
        if matches.is_present("debug") {print_debug_msg(bing_url[0].as_str())}
        fileman = misc::download_wallpapers(bing_url.to_vec(), savepath, Option::from(true));
    }

    if wallheaven{
        let wallheaven_username = &wallheaven_creds.username;
        let wallheaven_coll_id = wallheaven_creds.coll_id;

        let col = get_wallheaven(wallheaven_coll_id, &wallheaven_username);

        fileman = misc::download_wallpapers(col.to_owned(), savepath, Option::from(false));
    }



    if matches.is_present("only"){

        //only use downloaded/remote wallpapers.
        file_manifest = fileman;

    }else {
        for f in update_file_list(savepath) {
            file_manifest.push(f)
        }
    }


    return file_manifest;
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
    fn wallheaven_get_wallpapers() {
        let res = super::wallheaven::wallheaven_getcoll("th4n0s", 655812);
        assert_eq!(res.is_ok(),true)
    }

}