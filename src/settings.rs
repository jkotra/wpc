use clap::ArgMatches;
use log::{info};

use crate::misc::secs_till_next_hour;

#[derive(Default, Debug)]
pub struct RedditOptions {
    pub reddit: String,
    pub reddit_n: i64,
    pub reddit_sort: String,
    pub reddit_min_height: u32,
    pub reddit_min_width: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct ThemeOptions {
    pub set_theme: bool,
    pub force_dark_theme: bool,
    pub theme_th: f32,
    pub theme_dark_only: bool,
    pub theme_light_only: bool,
}

#[derive(Debug)]
pub struct WPCSettings {
    pub directory: String,
    pub interval: u64,
    pub update: u64,
    pub maxage: i64,

    /* global application flags */
    pub startup: bool,
    pub background: bool,
    pub grayscale: bool,

    pub theme_options: ThemeOptions,

    /* plugin flags */
    pub wallhaven: bool,
    pub reddit: bool,
    pub reddit_options: RedditOptions,
    pub local: bool,
    pub dynamic: bool,
    pub dynamic_config_file: String
}

pub fn parse(matches: ArgMatches) -> WPCSettings {

    let mut settings: WPCSettings = WPCSettings { 
    directory: matches.value_of("directory").unwrap().to_string(),
    interval: matches.value_of("interval").unwrap().parse().unwrap(),
    update: matches.value_of("update").unwrap().parse().unwrap(),
    maxage: if matches.occurrences_of("maxage") > 0 { matches.value_of("maxage").unwrap().parse().unwrap() } else { -1 },
    startup: matches.occurrences_of("startup") > 0,
    background: matches.occurrences_of("background") > 0,
    grayscale: matches.occurrences_of("grayscale") > 0,
    theme_options: ThemeOptions { 
    set_theme: matches.occurrences_of("set-theme") > 0,
    force_dark_theme: false,
    theme_th: matches.value_of("theme-threshold").unwrap().parse().unwrap(),
    theme_dark_only: matches.occurrences_of("theme-dark") > 0,
    theme_light_only: matches.occurrences_of("theme-light") > 0,
    },
    wallhaven: matches.occurrences_of("wallhaven") > 0,
    reddit: matches.occurrences_of("reddit") > 0,
    reddit_options: RedditOptions { reddit: matches.value_of("reddit").unwrap().to_string(), reddit_n: matches.value_of("reddit-n").unwrap().parse().unwrap(), reddit_sort: matches.value_of("reddit-sort").unwrap().to_string(), reddit_min_height: matches.value_of("reddit-min-height").unwrap().parse().unwrap(), reddit_min_width: matches.value_of("reddit-min-width").unwrap().parse().unwrap() },
    local: matches.occurrences_of("local") > 0,
    dynamic: matches.occurrences_of("dynamic") > 0,
    dynamic_config_file: std::fs::canonicalize(matches.value_of("dynamic").unwrap()).unwrap().to_str().unwrap().to_owned()
    };

    if !settings.wallhaven && !settings.reddit && !settings.local{
        settings.local = true;
        info!("no flags set! setting local = true.")
    }

    if settings.directory == "." {
        settings.directory = String::from(std::env::current_dir().unwrap().to_str().unwrap());
        info!("expanded . to {}", settings.directory);
    }

    if settings.theme_options.set_theme {
        if settings.theme_options.theme_th > 100.0{
            settings.theme_options.theme_th = 100.0
        }
    }

    if settings.dynamic {
        settings.update = secs_till_next_hour() as u64;
        settings.interval = settings.update;
    }


    return settings;
}