use std::io;
use std::fs::File;
use std::path::PathBuf;
extern crate rand;
use rand::Rng;

use log::{debug, error};

use std::env::current_exe;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

extern crate notify;

use notify::{RecommendedWatcher, Watcher, RecursiveMode};
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender};
use std::time::Duration;


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


#[cfg(test)]
mod misc_tests {

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

}