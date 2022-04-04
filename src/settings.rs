use clap::ArgMatches;
use log::{info};

#[derive(Default, Debug)]
pub struct RedditOptions {
    pub reddit: String,
    pub reddit_n: i64,
    pub reddit_sort: String,
    pub reddit_min_height: u32,
    pub reddit_min_width: u32,
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

    /* plugin flags */
    pub wallhaven: bool,
    pub reddit: bool,
    pub reddit_options: RedditOptions,
    pub local: bool,
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
    wallhaven: matches.occurrences_of("wallhaven") > 0,
    reddit: matches.occurrences_of("reddit") > 0,
    reddit_options: RedditOptions { reddit: matches.value_of("reddit").unwrap().to_string(), reddit_n: matches.value_of("reddit-n").unwrap().parse().unwrap(), reddit_sort: matches.value_of("reddit-sort").unwrap().to_string(), reddit_min_height: matches.value_of("reddit-min-height").unwrap().parse().unwrap(), reddit_min_width: matches.value_of("reddit-min-width").unwrap().parse().unwrap() },
    local: matches.occurrences_of("local") > 0,
    };

    if !settings.wallhaven && !settings.reddit && !settings.local{
        settings.local = true;
        info!("no flags set! setting local = true.")
    }

    if settings.directory == "." {
        settings.directory = String::from(std::env::current_dir().unwrap().to_str().unwrap());
        info!("expanded . to {}", settings.directory);
    }


    return settings;
}