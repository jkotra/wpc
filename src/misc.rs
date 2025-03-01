use std::fs::File;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
extern crate rand;
use chrono::Timelike;
use rand::Rng;
use serde::{Deserialize, Serialize};

use log::{debug, error, info};

use std::env::current_exe;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

extern crate notify;

use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::time::Duration;

use crate::settings::{ThemeOptions, TriggerArg, TriggerConfig};

#[derive(Debug, Serialize, Deserialize)]
pub struct SingleConfig {
    pub hour: u32,
    pub path: String,
    pub darkmode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DynamicConfig {
    pub configs: Vec<SingleConfig>,
}

pub fn notify_event(dir: std::sync::Arc<PathBuf>, main_thread_tx: Sender<bool>) -> () {
    //let dir = dir.as_str();

    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2)).unwrap();
    watcher
        .watch(dir.as_ref(), RecursiveMode::NonRecursive)
        .unwrap();

    loop {
        match rx.recv() {
            Ok(event) => {
                debug!("event received: {:?}", event);
                match event {
                    notify::DebouncedEvent::Create(_)
                    | notify::DebouncedEvent::Remove(_)
                    | notify::DebouncedEvent::Rename(_, _) => {
                        main_thread_tx.send(true).unwrap();
                    }
                    _ => (),
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

pub fn wait(sec: u64) {
    std::thread::sleep(std::time::Duration::from_secs(sec));
}

pub fn get_wpc_args() -> Vec<String> {
    let prohibited = vec!["--startup", "-S", "--background"];
    let canonicalize_args = vec!["-d", "--directory", "--trigger", "--dynamic"];

    let mut args: Vec<String> = std::env::args()
        .filter(|arg| !prohibited.contains(&arg.as_str()))
        .collect();
    for i in 0..args.len() {
        if canonicalize_args.contains(&args[i].as_str()) {
            args[i + 1] = std::fs::canonicalize(PathBuf::from(&args[i + 1]))
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
        }
    }

    debug!("wpc args: {:?}", args);
    args
}

pub fn run_in_background() {
    let mut args = get_wpc_args();
    args.remove(0); //remove executable name

    #[cfg(target_os = "windows")]
    let _child = std::process::Command::new(current_exe().unwrap().to_str().unwrap())
        .args(&args)
        .creation_flags(0x08000000) //CREATE_NO_WINDOW
        .spawn()
        .expect("Child process failed to start.");

    #[cfg(target_os = "linux")]
    let _child = std::process::Command::new(current_exe().unwrap().to_str().unwrap())
        .args(&args)
        .spawn()
        .expect("Child process failed to start.");
}

pub async fn download_wallpapers(urls: Vec<String>, savepath: &PathBuf) -> Vec<String> {
    let mut remote_files: Vec<String> = vec![];

    for url in urls {
        let file_vec: Vec<&str>;

        file_vec = url.split("/").collect();

        let filename = savepath.join(file_vec[file_vec.len() - 1]);
        remote_files.push(String::from(filename.to_str().unwrap()));

        match async_download(url.as_str(), filename.to_str().unwrap()).await {
            Ok(_) => (),
            Err(why) => error!("Error: {:?}", why),
        }
    }

    return remote_files;
}

async fn async_download(url: &str, filename: &str) -> Result<bool, String> {
    let filedest = PathBuf::from(filename);
    if filedest.exists() {
        return Ok(true);
    }
    let response = match reqwest::get(url).await {
        Ok(f) => f,
        Err(why) => return Err(String::from(format!("{:?}", why))),
    };

    let mut out = File::create(filedest).expect("failed to create file");
    let content = match response.bytes().await {
        Ok(f) => f,
        Err(why) => return Err(String::from(format!("{:?}", why))),
    };

    let mut content = std::io::Cursor::new(content);
    io::copy(&mut content, &mut out).expect("failed to copy content");

    return Ok(true);
}

pub fn random_n(len_max: usize) -> usize {
    let mut rng = rand::thread_rng();
    if len_max == 1 {
        return 0;
    }
    rng.gen_range(0, len_max)
}

pub fn update_file_list(dirpath: &PathBuf, maxage: i64) -> Vec<String> {
    let mut file_list = std::fs::read_dir(dirpath)
        .unwrap()
        .map(|f| f.unwrap().path().to_string_lossy().to_string())
        .collect();
    if maxage != -1 {
        file_list = maxage_filter(file_list, maxage);
    }
    file_list
        .into_iter()
        .filter(|f| f.ends_with(".png") || f.ends_with(".jpg") || f.ends_with(".jpeg"))
        .collect()
}

pub fn maxage_filter(file_list: Vec<String>, maxage: i64) -> Vec<String> {
    if maxage == -1 {
        return file_list;
    }

    let mut filtered: Vec<String> = vec![];

    for file in file_list {
        //current time as timestamp
        let maxage_time = chrono::Local::now().timestamp() - i64::from(maxage * 60 * 60);

        //get created date and convert to timestamp.
        let f_ct = std::fs::metadata(&file)
            .unwrap()
            .created()
            .unwrap()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if maxage_time as u64 > f_ct {
            continue;
        } else {
            filtered.push(file)
        }
    }

    debug!("time filtered: {:?}", filtered);
    return filtered;
}

pub fn is_image_min_dim(file_path: &str, height: Option<u32>, width: Option<u32>) -> bool {
    let img = image::open(file_path).unwrap();
    let (i_width, i_height) = image::GenericImageView::dimensions(&img);

    if height.is_some() {
        if i_height < height.unwrap() {
            return false;
        }
    }

    if width.is_some() {
        if i_width < width.unwrap() {
            return false;
        }
    }

    return true;
}

pub fn is_linux_gnome_de() -> bool {
    let res = std::env::var("DESKTOP_SESSION").unwrap().to_string();
    debug!("DESKTOP_SESSION = {}", res);
    if res.contains("gnome") {
        return true;
    }
    return false;
}

pub fn brighness_score(wp: &str) -> i64 {
    let im = image::open(wp).unwrap();
    let rgb = im.to_rgba8();
    let pix = rgb.pixels();

    let mut total: i64 = 0;
    let len = pix.len() as i64;

    for p in pix {
        //println!("{:?}", p.0);
        total += p.0[0] as i64;
        total += p.0[1] as i64;
        total += p.0[2] as i64;
    }

    let mean = total / len;

    if mean == 0 {
        return 0;
    }

    return mean * 100 / (255 * 3);
}

pub fn get_dynamic_wp(config_file: &str) -> Option<SingleConfig> {
    let t = chrono::offset::Local::now();

    let config = std::path::PathBuf::from_str(config_file).unwrap();

    if !config.exists() {
        // can files and create json
        let mut files = update_file_list(&config.parent().unwrap().to_path_buf(), -1);
        files.sort();
        let mut interval = 23 / files.len();
        let mut hour = 0;
        if interval < 1 {
            interval = 1
        };
        info!("calc i={} h={}", interval, hour);
        let mut generated: Vec<SingleConfig> = Vec::new();

        for f in files {
            let c = SingleConfig {
                hour: hour as u32,
                path: f,
                darkmode: Some(false),
            };
            generated.push(c);
            hour += interval;
            debug!("generated SingleConfig");
        }

        let stub = std::fs::File::create(config.clone());
        let writer = std::io::BufWriter::new(stub.unwrap());
        let content = DynamicConfig { configs: generated };

        match serde_json::to_writer_pretty(writer, &content) {
            Ok(()) => info!("config file generated!"),
            Err(err) => {
                error!("{:?}", err);
                return None;
            }
        };
    }

    let data = match std::fs::read_to_string(config.clone()) {
        Ok(s) => s,
        Err(err) => {
            error!("{:?}", err);
            return None;
        }
    };

    let d: DynamicConfig = match serde_json::from_str(data.as_str()) {
        Ok(d) => d,
        Err(err) => {
            error!("{:?}", err);
            return None;
        }
    };

    let mut wp: Option<SingleConfig> = None;

    for mut c in d.configs {
        debug!("c.path={} c.hour={} t.hour={}", c.path, c.hour, t.hour());
        if c.hour <= t.hour() {
            // check if image exists at path of *.json path
            let pbuf = config.parent().unwrap().join(&c.path);
            if pbuf.exists() {
                c.path = pbuf.to_str().unwrap().to_string();
                wp = Some(c)
            }
        }
    }

    log::info!("selected dynamic config = {:?}", wp);

    return wp;
}

pub fn secs_till_next_hour() -> u64 {
    let t = chrono::offset::Local::now();
    let min = 60 - t.minute();

    let next_hr = t + chrono::Duration::minutes(min as i64);
    let left = next_hr.timestamp() - t.timestamp();

    debug!("secs left untl next hour = {}", left);

    return left as u64;
}

pub fn to_grayscale(wallpaper: PathBuf) -> PathBuf {
    let mut gs_pf = PathBuf::from(std::env::temp_dir());
    gs_pf.push(wallpaper.file_name().unwrap());

    let img = image::open(wallpaper).unwrap();

    //convert to grayscale
    let img = image::imageops::grayscale(&img);

    img.save(gs_pf.clone()).unwrap();
    gs_pf
}

pub fn load_trigger_config<P>(config_file: P) -> Option<TriggerConfig>
where
    P: AsRef<Path>,
{
    let config_str = std::fs::read_to_string(config_file.as_ref()).unwrap();
    Some(serde_json::from_str(&config_str).unwrap())
}

fn map_trigger_arg_values(
    arg: &TriggerArg,
    wallpaper: &PathBuf,
    theme_options: &ThemeOptions,
) -> String {
    match arg {
        TriggerArg::Brightness => brighness_score(wallpaper.to_str().unwrap()).to_string(),
        TriggerArg::Grayscale => theme_options.grayscale.to_string(),
        TriggerArg::ThemeDarkOnly => theme_options.theme_dark_only.to_string(),
        TriggerArg::ThemeLightOnly => theme_options.theme_light_only.to_string(),
    }
}

pub fn run_trigger(wallpaper: PathBuf, theme_options: &ThemeOptions, config: &TriggerConfig) {
    if !config.enabled {
        return;
    }

    let bin = std::path::Path::new(&config.bin);
    let mut args: Vec<String> = config
        .args
        .iter()
        .map(|arg| map_trigger_arg_values(arg, &wallpaper, theme_options))
        .collect();
    args.insert(0, config.file.clone());

    let out = std::process::Command::new(bin).args(args).output().unwrap();
    debug!("trigger stdout: {}", String::from_utf8_lossy(&out.stdout));
}

#[cfg(test)]
mod misc_tests {

    use std::{path::PathBuf, process::Output, str::FromStr};

    use chrono::Timelike;
    use image::{ImageBuffer, RgbImage};

    use crate::settings::{ThemeOptions, TriggerArg, TriggerConfig};

    use super::{get_dynamic_wp, run_trigger, update_file_list};

    #[tokio::test]
    async fn async_download_test() {
        let mut url = vec![];
        url.push(
            String::from("https://upload.wikimedia.org/wikipedia/commons/thumb/8/80/Wikipedia-logo-v2.svg/1024px-Wikipedia-logo-v2.svg.png")
    );

        let files =
            super::download_wallpapers(url, &std::path::PathBuf::from("./target/debug")).await;
        assert_eq!(files.len(), 1 as usize);

        let test_file_path = std::path::PathBuf::from(&files[0]);
        assert_eq!(test_file_path.exists(), true);
    }

    fn gen_dummy_images() -> PathBuf {
        // generate dummy images
        let img: RgbImage = ImageBuffer::new(128, 128);

        let pbuf = std::path::PathBuf::from_str("tests").unwrap();
        if !pbuf.exists() {
            std::fs::create_dir(pbuf.clone()).unwrap();
        }

        img.save("tests/1.jpg").unwrap();
        img.save("tests/2.jpg").unwrap();

        return pbuf.canonicalize().unwrap();
    }

    #[test]
    fn dynamic_wallpaper_test() {
        let t = chrono::Local::now();

        let test_root = gen_dummy_images();

        let wp_config_path = test_root.join("config.json");

        if wp_config_path.exists() {
            std::fs::remove_file(wp_config_path.clone()).unwrap();
        };

        let chosen = get_dynamic_wp(wp_config_path.to_str().unwrap());

        assert_eq!(chosen.is_some(), true);

        let chosen = chosen.unwrap();

        if t.hour() >= 11 {
            assert_eq!(chosen.path.ends_with("2.jpg"), true);
        } else {
            assert_eq!(chosen.path.ends_with("1.jpg"), true);
        }
    }

    #[test]
    fn local_mode() {
        gen_dummy_images();
        let files = update_file_list(&std::fs::canonicalize("tests").unwrap(), -1);
        assert_eq!(files.len(), 2);
    }

    fn get_python_bin() -> std::io::Result<Output> {
        #[cfg(target_os = "windows")]
        return std::process::Command::new("where").arg("python").output();

        #[cfg(target_os = "linux")]
        return std::process::Command::new("which").arg("python").output();
    }

    #[test]
    fn trigger_on_wallpaper_change() {
        let mut python_bin = match get_python_bin() {
            #[cfg(target_os = "linux")]
            Ok(out) => String::from_utf8_lossy(&out.stdout).trim_end().to_string(),
            #[cfg(target_os = "windows")]
            Ok(out) => String::from_utf8_lossy(&out.stdout)
                .split("\n")
                .nth(0)
                .unwrap()
                .trim_end()
                .to_string(),
            Err(why) => panic!("Unable to get python path: {:?}", why),
        };

        if std::env::var("GCP_CLOUD_BUILD").is_ok() {
            python_bin = "/usr/bin/python3".to_string();
        }

        let mut python_test_trigger = String::from("import sys");
        python_test_trigger += "\nwith open('tests//output.txt', 'w+') as f:\n";
        python_test_trigger += "\tf.write(f'OK {sys.argv[1]}')";

        match std::fs::write("tests/trigger.py", python_test_trigger) {
            Ok(_) => (),
            Err(why) => panic!("Unable to write test file: {:?}", why),
        }

        gen_dummy_images();

        let tc = TriggerConfig {
            enabled: true,
            bin: python_bin,
            file: std::fs::canonicalize("tests/trigger.py")
                .unwrap()
                .to_string_lossy()
                .to_string(),
            args: vec![TriggerArg::Brightness],
        };
        let to = ThemeOptions {
            ..Default::default()
        };
        let wallpaper = std::path::PathBuf::from("tests/1.jpg");

        run_trigger(wallpaper, &to, &tc);

        match std::fs::read("tests/output.txt") {
            Ok(fc) => {
                let content = String::from_utf8_lossy(&fc).to_string();
                assert_eq!(content, "OK 0");
            }
            Err(why) => panic!("Unable to read: {:?}", why),
        }
    }
}
