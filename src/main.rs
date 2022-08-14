#[macro_use]
extern crate clap;

use clap::App;
use image;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::sync::mpsc::channel;

use log::{debug, info};

#[allow(unused_imports)]
use crate::misc::*;

mod misc;

mod changer;
use changer::change_wallpaper;

mod web;
use web::wallhaven::WallHaven;
use web::reddit::Reddit;

mod settings;
use settings::ThemeOptions;

#[tokio::main]
async fn main() {

    env_logger::init();

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let app_settings = settings::parse(matches);

    info!("{:?}", app_settings);

    if cfg!(linux) {
        if !misc::is_linux_gnome_de() {
            panic!("DE not supported!");
        }
    }

    if app_settings.startup {
        changer::add_to_startup();
    }

    if app_settings.background {
        misc::run_in_background();
        std::process::exit(0);
    }

    /* setup wallhaven */
    let mut wallhaven_cc = WallHaven {
        ..Default::default()
    };

    if app_settings.wallhaven {
        info!("loading wallhaven...");
        let wallhaven_json_file = std::path::PathBuf::from(app_settings.directory.clone()).join("wallhaven.json");
        debug!("wallhaven_json_file = {:?} | exists = {}", wallhaven_json_file, wallhaven_json_file.exists());
        wallhaven_cc.init(wallhaven_json_file);
    }

    /* setup reddit */
    let mut reddit_com = Reddit{ ..Default::default() };
    if app_settings.reddit{
        reddit_com.subreddit = app_settings.reddit_options.reddit;
        reddit_com.n = app_settings.reddit_options.reddit_n;
        reddit_com.min_height = app_settings.reddit_options.reddit_min_height;
        reddit_com.min_width = app_settings.reddit_options.reddit_min_width;
        reddit_com.cat = app_settings.reddit_options.reddit_sort;
    }
    

    /* Inital while loop until we have atleast 1 image */
    let mut time_since = std::time::Instant::now();
    let mut candidates: Vec<String> = vec![];

    while candidates.len() == 0{

        if app_settings.local{
            candidates = misc::update_file_list(&app_settings.directory, app_settings.maxage);
        }

        if app_settings.wallhaven{
            let mut files = wallhaven_cc.update(&app_settings.directory, app_settings.maxage).await;
            candidates.append(&mut files);
        }

        if app_settings.reddit{
            let mut files = reddit_com.update(&app_settings.directory, app_settings.maxage).await;
            candidates.append(&mut files);
        }

        info!("Initial candidates = {}", candidates.len());
        wait(1);
    }

    
    let watch_dir = std::sync::Arc::new(app_settings.directory.clone());
    let (tx, rx) = channel();
    std::thread::spawn(move || {
        let watch_dir = watch_dir.clone();
        misc::notify_event(watch_dir, tx);   
    });

    
    // main loop
    loop {
        
        if app_settings.local {
        match rx.try_recv() {
            Ok(_) => { candidates = misc::update_file_list(&app_settings.directory, app_settings.maxage) },
            Err(_) => ()
        }

        debug!("candidates = {}", candidates.len());
    }

        //TODO: change wallpaper 
        change_wallpaper_random(&candidates, app_settings.grayscale, app_settings.theme_options);
        
        wait(app_settings.interval);
        info!("sleeping for {} secs...", app_settings.interval);

        if time_since.elapsed().as_secs() >= app_settings.update {

            let mut candidates: Vec<String> = vec![];

            if app_settings.wallhaven {
                let mut files = wallhaven_cc.update(&app_settings.directory, app_settings.maxage).await;
                candidates.append(&mut files);
            }

            if app_settings.reddit {
                let mut files = reddit_com.update(&app_settings.directory, app_settings.maxage).await;
                candidates.append(&mut files);
            }

            time_since = std::time::Instant::now();
            info!("updated candidates = {}", candidates.len());
        }
    }
}

fn change_wallpaper_random(file_list: &Vec<String>, is_grayscale: bool, theme_options: ThemeOptions) {

    let rand_n = random_n(file_list.len());
    let wp = file_list.get(rand_n).unwrap();

    let mut wallpaper = PathBuf::from(wp);
    let wallpaper_ext = wallpaper.extension().and_then(OsStr::to_str).unwrap();

    debug!("extension = {}", wallpaper_ext);

    if is_grayscale{
        info!("applying grayscale...");
        let mut gs_pf = PathBuf::from(std::env::temp_dir());
        gs_pf.push(format!("gs.{}", wallpaper_ext));
        let img = image::open(wallpaper).unwrap();
        //convert to grayscale
        let img = image::imageops::grayscale(&img);
        img.save(gs_pf.to_str().unwrap()).unwrap();
        wallpaper = gs_pf.clone();
    }
        
    
    info!("setting wallpaper = {:?}", wallpaper);

    change_wallpaper(wallpaper.to_str().unwrap(), theme_options);

}