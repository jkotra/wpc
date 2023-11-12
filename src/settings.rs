use crate::web::reddit::RedditSort;
use clap::{Args, Parser};
use log::info;
use serde::Deserialize;
use std::path::PathBuf;
use strum_macros::Display;

use crate::misc::{load_trigger_config, secs_till_next_hour};

#[derive(Default, Debug, Args)]
pub struct RedditOptions {
    #[arg(
        name = "subreddit",
        long,
        required = false,
        default_value = "wallpaper"
    )]
    pub reddit: String,
    #[arg(long, name = "reddit-n", required = false, default_value_t = 6)]
    pub reddit_n: i64,
    #[arg(
        value_enum,
        long,
        name = "reddit-sort",
        required = false,
        default_value_t = RedditSort::Hot,
    )]
    pub reddit_sort: RedditSort,
    #[arg(
        long,
        name = "reddit-min-height",
        required = false,
        default_value_t = 1920
    )]
    pub reddit_min_height: u32,
    #[arg(
        long,
        name = "reddit-min-width",
        required = false,
        default_value_t = 1080
    )]
    pub reddit_min_width: u32,
}

#[derive(Debug, Args, Default, Copy, Clone)]
pub struct ThemeOptions {
    #[arg(long, name = "set-theme", default_value_t = false)]
    pub set_theme: bool,
    #[arg(long, default_value_t = false)]
    pub grayscale: bool,
    #[arg(long, name = "force-dark-theme", default_value_t = false)]
    pub force_dark_theme: bool,
    #[arg(long, name = "theme-brigness-threshold", default_value_t = 50.0)]
    pub theme_th: f32,
    #[arg(long, name = "dark-theme-only", default_value_t = false)]
    pub theme_dark_only: bool,
    #[arg(long, name = "light-theme-only", default_value_t = false)]
    pub theme_light_only: bool,
}

#[derive(Debug, Deserialize, Display)]
pub enum TriggerArg {
    Grayscale,
    Brightness,
    ThemeDarkOnly,
    ThemeLightOnly,
}

#[derive(Debug, Default, Deserialize)]
pub struct TriggerConfig {
    pub enabled: bool,
    pub bin: String,
    pub file: String,
    pub args: Vec<TriggerArg>,
}

#[derive(Debug, Parser)]
#[command(author, version, long_about)]
pub struct WPCSettings {
    #[arg(long, short = 'd', help = "save / source directory.")]
    pub directory: PathBuf,

    #[arg(
        short = 'c',
        long = "change-interval",
        name = "c_seconds",
        visible_alias = "interval",
        visible_short_alias = 'i',
        help = "interval between wallpaper change.",
        default_value_t = 300
    )]
    pub interval: u64,

    #[arg(
        short = 'f',
        name = "f_seconds",
        long = "fetch-interval",
        visible_alias = "update",
        visible_short_alias = 'u',
        help = "interval between each refresh from configures sources.",
        default_value_t = 3600
    )]
    pub update: u64,

    #[arg(long, required=false, help = "maximum age of wallpaper.", default_value_t = -1)]
    pub maxage: i64,

    /* global application flags */
    #[arg(
        long,
        short = 's',
        visible_short_alias = 'S',
        required = false,
        help = "add WPC to startup.",
        default_value_t = false
    )]
    pub startup: bool,
    #[arg(long, help = "remove WPC from startup.", default_value_t = false)]
    pub rm_startup: bool,
    #[arg(
        long,
        short = 'b',
        help = "run WPC as background process.",
        default_value_t = false
    )]
    pub background: bool,

    #[command(flatten)]
    pub theme_options: ThemeOptions,
    #[arg(long = "trigger", required = false)]
    pub trigger_config_file: Option<PathBuf>,
    #[arg(skip)]
    pub trigger_config: TriggerConfig,

    /* plugin flags */
    #[arg(short = 'w', help = "wallhaven.cc plugin.", default_value_t = false)]
    pub wallhaven: bool,
    #[arg(long, short = 'r', help = "reddit plugin.", default_value_t = false)]
    pub reddit: bool,
    #[command(flatten)]
    pub reddit_options: RedditOptions,
    #[arg(
        long,
        short = 'l',
        required = false,
        help = "Include only local files.",
        default_value_t = false
    )]
    pub local: bool,
    #[arg(
        long = "dynamic",
        required = false,
        help = "Dynamically set wallpaper based on time.",
        default_value = None
    )]
    pub dynamic_config_file: Option<PathBuf>,
    #[arg(skip)]
    pub dynamic: bool,
}

pub fn parse() -> WPCSettings {
    let mut cli = WPCSettings::parse();
    if !cli.wallhaven && !cli.reddit {
        cli.local = true;
    }

    cli.directory = cli.directory.canonicalize().unwrap();

    if (cli.theme_options.theme_th > 100.0) || (cli.theme_options.theme_th < 0.0) {
        cli.theme_options.theme_th = 50.0
    }
    match cli.dynamic_config_file {
        Some(_) => {
            cli.dynamic = true;
            cli.dynamic_config_file =
                Some(cli.dynamic_config_file.unwrap().canonicalize().unwrap());
            cli.update = secs_till_next_hour();
            cli.interval = cli.update;
        }
        None => cli.dynamic = false,
    }

    match cli.trigger_config_file {
        Some(f) => {
            cli.trigger_config_file = Some(f.clone().canonicalize().unwrap());
            cli.trigger_config = load_trigger_config(f.clone()).unwrap_or(TriggerConfig {
                ..Default::default()
            })
        }
        None => (),
    }

    info!("{:#?}", cli);
    cli
}
