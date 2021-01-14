#[macro_use]
extern crate clap;

use clap::App;
use image;


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
use bing::Bing;

#[path = "web/wallhaven.rs"]
mod wallhaven;
use wallhaven::WallHaven;

#[path = "web/reddit.rs"]
mod reddit;
use reddit::Reddit;

#[tokio::main]
async fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    
    let mut is_gs = matches.occurrences_of("grayscale") != 0;
    
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

    let mut local_flag = false;
    let bing_flag = matches.is_present("bing");
    let wallhaven_flag = matches.is_present("wallhaven");
    let reddit_flag = matches.occurrences_of("reddit") != 0;
    
    let user_interval = matches
        .value_of("interval")
        .unwrap()
        .parse::<u64>()
        .unwrap();
    let user_update_interval = matches.value_of("update").unwrap().parse::<u64>().unwrap();
    

    
    if !wallhaven_flag && !bing_flag && !reddit_flag {
        local_flag = true;
    }


    let is_gs_rm = matches.occurrences_of("rm-grayscale-files") != 0;
    if is_gs_rm{ 
        misc::clean_gs(savepath);
        is_gs = false }

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

    /* setup wallhaven */
    let mut wallhaven_cc = WallHaven {
        ..Default::default()
    };

    if wallhaven_flag {
        wallhaven_cc.init(savepath);
        wallhaven_cc = wallhaven_cc.read_json(savepath);
    }
    /* END */

    /* setup reddit */

    let reddit_com = Reddit{ subreddit: String::from(matches.value_of("reddit").unwrap()),
                            n: matches.value_of("reddit-n").unwrap().parse::<i64>().unwrap(),
                            cat: String::from(matches.value_of("reddit-sort").unwrap()),
                            min_width: matches.value_of("reddit-min-width").unwrap().parse::<u32>().unwrap(),
                            min_height: matches.value_of("reddit-min-height").unwrap().parse::<u32>().unwrap() };

    /* end */
    

    /* Inital while loop until we have atleast 1 image */
    
    let mut candidates: Vec<String> = vec![];

    while candidates.len() == 0{

        main_debug.debug(
            format!("flags: local={}, wallhaven={}, bing={}, reddit={}", local_flag, wallhaven_flag, bing_flag, reddit_flag)
        );

        if local_flag{
            candidates = update_local_files(savepath, maxage, &main_debug);
        }

        if wallhaven_flag{
            let mut files = wallhaven_cc.update(savepath, maxage, &main_debug).await;
            candidates.append(&mut files);
        }

        if reddit_flag{
            let mut files = reddit_com.update(savepath, maxage, &main_debug).await;
            candidates.append(&mut files);
        }

        if bing_flag{
            let mut files = Bing.update(savepath, &main_debug).await;
            candidates.append(&mut files);
        }
    }
    
    // main loop
    loop {

        //change wallpaper 
        change_wallpaper_random(&candidates, is_gs, &main_debug);

        wait(user_interval);

        main_debug.debug(
            format!("update_interval = {} elapsed_since = {}", user_update_interval, time_since.elapsed().as_secs())
        );

        if time_since.elapsed().as_secs() >= user_update_interval {

            let mut candidates: Vec<String> = vec![];
            
            if local_flag {
                candidates = update_local_files(savepath,
                                                maxage, &main_debug);
            }

            if wallhaven_flag {
                let mut files = wallhaven_cc.update(savepath, maxage, &main_debug).await;
                candidates.append(&mut files);
            }

            if bing_flag{
                let mut files = Bing.update(savepath, &main_debug).await;
                candidates.append(&mut files);
            }

            if reddit_flag{
                let mut files = reddit_com.update(savepath, maxage, &main_debug).await;
                candidates.append(&mut files);
            }

            time_since = std::time::Instant::now();
        }
    }
}

fn change_wallpaper_random(file_list: &Vec<String>, gs: bool, wpc_debug: &WPCDebug) {

    let rand_n = random_n(file_list.len());
    let wp = file_list.get(rand_n).unwrap();

    let mut wp_to_set = std::path::PathBuf::from(&wp);

    let wp_filename: Vec<&str> = wp.split("/").collect();
    let wp_filename = wp_filename[(wp_filename.len() - 1) as usize];
    let mut wp_filename: Vec<&str> = wp_filename.split(".").collect();

    let wp_ext = wp_filename.pop().unwrap();
    let wp_name = wp_filename.join("");
    
    wpc_debug.debug(format!("Total = {} rand_n = {}", file_list.len(), rand_n));

    if gs{
        
        let mut wp_pbuf_gs = wp_to_set.clone();
        wp_pbuf_gs.pop();
        wp_pbuf_gs.push("grayscale");

        if !wp_pbuf_gs.exists(){
            let _ = std::fs::create_dir(wp_pbuf_gs.to_str().unwrap());
        }
        
        //push filename
        wp_pbuf_gs.push(String::from(wp_name) + "_grayscale." + wp_ext);

        if !wp_pbuf_gs.exists(){
            //open
            let img = image::open(wp).unwrap();

            //convert to grayscale
            let img = image::imageops::grayscale(&img);

            //save
            img.save(wp_pbuf_gs.to_str().unwrap()).unwrap();
        }

        wpc_debug.debug(format!("grayscale Image = {}", wp_pbuf_gs.to_str().unwrap()));
        wp_to_set = wp_pbuf_gs.clone();

    }
        
    
    let wp =  wp_to_set.to_str().unwrap();

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
