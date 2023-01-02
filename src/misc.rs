use std::io;
use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;
extern crate rand;
use chrono::{Timelike};
use rand::Rng;
use serde::{Deserialize, Serialize};
use image::{GenericImageView, ImageBuffer};

use log::{debug, error, info};

use std::env::current_exe;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

extern crate notify;

use notify::{RecommendedWatcher, Watcher, RecursiveMode};
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender};
use std::time::Duration;


#[derive(Debug, Serialize, Deserialize)]
pub struct SingleConfig {
    pub hour: u32,
    pub path: String,
    pub darkmode: Option<bool>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DynamicConfig {
    pub configs: Vec<SingleConfig>
}

pub fn notify_event(dir: std::sync::Arc<String>, main_thread_tx: Sender<bool>,) -> () {
    let dir = dir.as_str();

    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2)).unwrap();
    watcher.watch(dir, RecursiveMode::NonRecursive).unwrap();

    loop {
        match rx.recv() {
            Ok(event) => {
                debug!("event received: {:?}", event);
            match event{
                notify::DebouncedEvent::NoticeWrite(_) => (),
                notify::DebouncedEvent::NoticeRemove(_) => (),
                notify::DebouncedEvent::Rescan => (),
                notify::DebouncedEvent::Error(_, _) => (),
                notify::DebouncedEvent::Write(_) => (),
                notify::DebouncedEvent::Chmod(_) => (),
                | notify::DebouncedEvent::Create(_)
                | notify::DebouncedEvent::Remove(_)
                | notify::DebouncedEvent::Rename(_, _) => {
                    main_thread_tx.send(true).unwrap();
                }
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

    let mut args: Vec<String> = std::env::args().collect();
    
    for i in (0..args.len()).rev(){
        if args[i] == "--startup"{ args.remove(i); }
        else if args[i] == "-S" { args.remove(i); }
        else if args[i] == "--background" { args.remove(i); }
        else if args[i] == "-d" || args[i] == "--directory" {
            if args[i+1] == "." {
                args[i+1] = String::from(std::env::current_dir().unwrap().to_str().unwrap());
            }
        }
    }

    debug!("wpc args: {:?}", args);
    return args;
}

pub fn run_in_background(){

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

pub async fn download_wallpapers(urls: Vec<String>, savepath: &str) -> Vec<String> {
    let mut remote_files: Vec<String> = vec![];

    for url in urls{
            let file_vec: Vec<&str>;

            file_vec = url.split("/").collect();
            

            let mut filename = PathBuf::from(savepath);
            filename = filename.join(file_vec[file_vec.len() - 1]);

            remote_files.push(String::from(filename.to_str().unwrap()));

            match async_download(url.as_str(), filename.to_str().unwrap()).await{
                Ok(_) => (),
                Err(why) => error!("Error: {:?}", why)
            }
        }

    return remote_files;
}


async fn async_download(url: &str, filename: &str) -> Result<bool, String> {

    let filedest = PathBuf::from(filename);
    if filedest.exists() { return  Ok(true); }
    let response = match reqwest::get(url).await{
        Ok(f) => f,
        Err(why) => return Err(String::from(format!("{:?}", why)))
    };

    let mut out = File::create(filedest).expect("failed to create file");
    let content = match response.bytes().await{
        Ok(f) => f,
        Err(why) => return Err(String::from(format!("{:?}", why)))
    };

    let mut content = std::io::Cursor::new(content);
    io::copy(&mut content, &mut out).expect("failed to copy content");

    return Ok(true)
}

pub fn random_n(len_max: usize) -> usize {
    let mut rng = rand::thread_rng();
    if len_max == 1 {return 0}
    rng.gen_range(0,len_max)
}

pub fn update_file_list(dirpath: &str, maxage: i64) -> Vec<String> {

    let files = std::fs::read_dir(dirpath).unwrap();
    let mut wallpapers: Vec<String> = vec![];
    let mut file_list = vec![];


    for file in files {
        let fp = file.unwrap().path().to_str().unwrap().to_string();
        file_list.push(fp)
    }

    file_list = maxage_filter(file_list, maxage);

    for file in file_list{

        if file.contains("_grayscale.") { continue }; //dont include grayscale images created by WPC
        if file.ends_with("png"){ wallpapers.push(file) }
        else if file.ends_with("jpg") { wallpapers.push(file) }
        else if file.ends_with("jpeg") { wallpapers.push(file) }
        else { continue }
    }

    return wallpapers
}

pub fn maxage_filter(file_list: Vec<String>, maxage: i64) -> Vec<String>{

    if maxage == -1 { return file_list }

    let mut filtered: Vec<String> = vec![];

    for file in file_list{

        //current time as timestamp
        let maxage_time = chrono::Local::now().timestamp() - i64::from(maxage * 60 * 60);

        //get created date and convert to timestamp.
        let f_ct = std::fs::metadata(&file).unwrap().created().unwrap().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();

        if maxage_time as u64 > f_ct{
            continue
        }
        else{
            filtered.push(file)
        }
    }
    
    debug!("time filtered: {:?}", filtered);
    return filtered

}


pub fn is_linux_gnome_de() -> bool {
    let res = std::env::var("DESKTOP_SESSION").unwrap().to_string();
    debug!("DESKTOP_SESSION = {}", res);
    if res.contains("gnome"){
        return true;
    }
    return false;
}

pub fn brighness_score(wp: &str) -> i64{

    let im = image::open(wp).unwrap();
    let rgb = im.to_rgba8();
    let pix = rgb.pixels();

    let mut total: i64 = 0;
    let len = pix.len() as i64;

    for p in pix{
        //println!("{:?}", p.0);
        total += p.0[0] as i64;
        total += p.0[1] as i64;
        total += p.0[2] as i64;
    }

    let mean = total / len;
    
    if mean == 0{
        return 0;
    }

    return mean * 100 / (255 * 3);

}

pub fn get_dynamic_wp(config_file: &str) -> Option<SingleConfig> {

    let t = chrono::offset::Local::now();

    let config = std::path::PathBuf::from_str(config_file)
    .unwrap();

    if !config.exists(){
        // can files and create json
        let mut files = update_file_list(config.parent().unwrap().to_str().unwrap(), -1);
        files.sort();
        let mut interval =  23 /  files.len();
        let mut hour = 0;
        if interval < 1 { interval = 1 };
        info!("calc i={} h={}", interval, hour);
        let mut generated: Vec<SingleConfig> = Vec::new();

        for f in files {
            let c = SingleConfig { hour: hour as u32, path: f, darkmode: Some(false) };
            generated.push(c);
            hour += interval;
            debug!("generated SingleConfig");
        }

        let stub = std::fs::File::create(config.clone());
        let writer = std::io::BufWriter::new(stub.unwrap());
        let content = DynamicConfig {configs: generated};
        
        match serde_json::to_writer_pretty(writer, &content){
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
            return None
        }
    };

    let d: DynamicConfig = match serde_json::from_str(data.as_str()){
        Ok(d) => d,
        Err(err) => {
            error!("{:?}", err);
            return None
        }
    };

    let mut wp: Option<SingleConfig> = None;

    for mut c in d.configs{
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

pub fn secs_till_next_hour() -> u32 {
    let t = chrono::offset::Local::now();
    let min = 59 - t.minute();
    
    let next_hr = t + chrono::Duration::minutes(min as i64);
    
    //let next_hr = next_hr - chrono::Duration::seconds(next_hr.second() as i64);

    let mut left = next_hr.timestamp() - t.timestamp();

    debug!("secs left = {}", left);
    
    if left == 0 {
        left = 60;
    };

    return left as u32;
}

#[cfg(test)]
mod misc_tests {
    use std::str::FromStr;

    use chrono::Timelike;
    use image::{ImageBuffer, RgbImage};

    use super::get_dynamic_wp;


    #[tokio::test]
    async fn async_download_test() {

        let mut url = vec![];
        url.push(
            String::from("https://upload.wikimedia.org/wikipedia/commons/thumb/8/80/Wikipedia-logo-v2.svg/1024px-Wikipedia-logo-v2.svg.png")
    ); 

        let files = super::download_wallpapers(url, "./target/debug").await;
        assert_eq!(files.len(), 1 as usize);
        
        let test_file_path = std::path::PathBuf::from(&files[0]);
        assert_eq!(test_file_path.exists(), true);
    }

    #[tokio::test]
    async fn dynamic_wallpaper_test() {

        let t = chrono::Local::now();
        let pbuf = std::fs::canonicalize("tests").unwrap();

        // generate dummy images
        let img: RgbImage = ImageBuffer::new(128, 128);
        img.save("tests/1.jpg").unwrap();
        img.save("tests/2.jpg").unwrap();

        let cfile = pbuf.clone().join("config.json");
        if cfile.exists() {
            std::fs::remove_file(cfile.clone()).unwrap();
        };

        let chosen = get_dynamic_wp(cfile.to_str().unwrap()).unwrap();
        if t.hour() <= 0 {
            assert_eq!(chosen.path, pbuf.join("1.jpg").to_str().unwrap().to_owned());
        }
        else{
            assert_eq!(chosen.path, pbuf.join("2.jpg").to_str().unwrap().to_owned());
        }
    }

}