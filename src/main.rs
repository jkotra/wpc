#[macro_use]
extern crate clap;
use clap::{App, ArgMatches};
use std::time::{Duration, Instant};
use crate::misc::{update_file_list, print_debug_msg};

#[path = "api/wallheaven.rs"] mod wallheaven;
#[path = "api/bing.rs"] mod bing;
mod misc;
mod change;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let debug = matches.occurrences_of("debug") != 0;
    let mut time = std::time::Instant::now();

    let mut wallpaper_manifest: Vec<String> = vec![];
    let mut file_manifest: Vec<String> = vec![];
    let savepath = matches.value_of("directory").unwrap();

    // check flags
    let bing_flag = matches.is_present("bing");

    let wallheaven_flag = matches.is_present("wallheaven_id") &&
        matches.is_present("wallheaven_username");

    let local_flag = matches.is_present("local");

    file_manifest = update(bing_flag,wallheaven_flag,local_flag,matches.clone(), savepath);

    // last check
    if file_manifest.len() == 0{
        panic!("No files added to file_manifest!")
    }

    loop {
        let wp = file_manifest.get(misc::random_n(file_manifest.len())).unwrap();
        if debug { print_debug_msg(wp) }
        if time.elapsed().as_secs() > matches.value_of("update").unwrap().parse::<u64>().unwrap() {

            if debug {print_debug_msg("Updating Images..") }
            // update stuff here
            let file_manifest = update(bing_flag,wallheaven_flag,local_flag,matches.to_owned(), savepath);
            time = Instant::now();
        }
        misc::wait(matches.value_of("interval").unwrap().parse::<u64>().unwrap());
        change::change_wallpaper_gnome(wp);
    }

}


fn get_bing() -> Vec<String> {
    let bing = bing::get_wallpaper_of_the_day();
    let bing = "https://bing.com".to_string()
        + bing.unwrap()["images"][0]["url"].as_str().unwrap().replace("&pid=hp","").as_str();
    return vec![bing]
}

fn get_wallheaven(collid: i64, username: &str) -> Vec<String> {
    let collection = wallheaven::wallheaven_getcoll("th4n0s", collid);
    let mut coll_urls: Vec<String> = vec![];

    for x in collection.unwrap()["data"].as_array() {
        for y in x {
            coll_urls.push(y["path"].as_str().unwrap().to_string())
        }
    }
    return coll_urls
}

fn update(bing: bool, wallheaven: bool, local: bool,matches: ArgMatches, savepath: &str) -> Vec<String>{
    let mut file_manifest: Vec<String> = vec![];

    if bing{

        let bing_url = get_bing();
        misc::download_wallpapers(bing_url.to_vec(), savepath);
        for f in update_file_list(savepath) {
            file_manifest.push(f);
        }
    }

    if wallheaven{
        let id = matches.value_of("wallheaven_id").unwrap().parse::<i64>();
        let col = get_wallheaven(id.unwrap(), matches.value_of("wallheaven_username").unwrap());
        misc::download_wallpapers(col.to_owned(), savepath);

        for f in update_file_list(savepath) {
            file_manifest.push(f)
        }
    }

    if local{
        for f in update_file_list(savepath){
            file_manifest.push(f)
        }
    }
    return file_manifest;
}