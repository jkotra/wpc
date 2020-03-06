#[macro_use]
extern crate clap;
use clap::{App, ArgMatches};
use std::time::{Duration, Instant};
use crate::misc::{update_file_list, print_debug_msg};

#[path = "api/wallheaven.rs"] mod wallheaven;
#[path = "api/bing.rs"] mod bing;

#[cfg(target_os = "windows")]
#[path = "api/windows.rs"] mod windows;

mod misc;
mod change;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let debug = matches.occurrences_of("debug") != 0;
    let mut time = std::time::Instant::now();

    // let mut wallpaper_manifest: Vec<String> = vec![];
    //let mut file_manifest: Vec<String> = vec![];

    let savepath = matches.value_of("directory").unwrap();

    // check flags
    let bing_flag = matches.is_present("bing");

    let wallheaven_flag = matches.is_present("wallheaven_id") &&
        matches.is_present("wallheaven_username");
    
    if wallheaven_flag {
    if !matches.is_present("wallheaven_id") ||
        !matches.is_present("wallheaven_username") {
        panic!("both wallheaven_id and wallheaven_username must be provided!")
    }
}

    let local_flag = matches.is_present("local");

    let file_manifest = update(bing_flag,wallheaven_flag,local_flag,matches.clone(), savepath);

    // last check
    if file_manifest.len() == 0{
        panic!("No files added to file_manifest!")
    }

    loop {
        let mut user_interval = matches.value_of("interval").unwrap().parse::<u64>().unwrap();
        let mut user_update_interval = matches.value_of("update").unwrap().parse::<u64>().unwrap();

        //only bing is the argument
        if matches.is_present("bing") &&
            !matches.is_present("wallheaven_id") &&
            !matches.is_present("local") {
            // set interval and update interval to 24 hrs
            user_update_interval = 60*60*24;
            user_interval = 60*60*24;
        }

        let wp = file_manifest.get(misc::random_n(file_manifest.len())).unwrap();
        if debug { print_debug_msg(wp) }

        #[cfg(target_os = "linux")]
        change::change_wallpaper_gnome(wp);

        #[cfg(target_os = "windows")]
        change::change_wallpaper_windows(wp);
        
        if time.elapsed().as_secs() > user_update_interval {

            if debug {print_debug_msg("Updating Images..") }
            // update stuff here
            let file_manifest = update(bing_flag,wallheaven_flag,local_flag,matches.to_owned(), savepath);
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

fn update(bing: bool, wallheaven: bool, local: bool,matches: ArgMatches, savepath: &str) -> Vec<String>{
    let mut file_manifest: Vec<String> = vec![];

    if bing{

        let bing_url = get_bing();
        if matches.is_present("debug") {print_debug_msg(bing_url[0].as_str())}
        misc::download_wallpapers(bing_url.to_vec(), savepath, Option::from(true));
        for f in update_file_list(savepath) {
            file_manifest.push(f);
        }
    }

    if wallheaven{
        let id = matches.value_of("wallheaven_id").unwrap().parse::<i64>();
        let col = get_wallheaven(id.unwrap(), matches.value_of("wallheaven_username").unwrap());
        misc::download_wallpapers(col.to_owned(), savepath, Option::from(false));

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