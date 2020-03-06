use std::time::{Duration, Instant};
use std::io::Write;
use std::io;
use std::fs::File;
use std::path::PathBuf;

extern crate rand;
use rand::Rng;
use chrono;

pub fn print_debug_msg(content: &str) {
    let now = std::time::SystemTime::now();
    println!("{:?}: {}", chrono::offset::Local::now(), content)
}

pub fn wait(sec: u64) {
    std::thread::sleep(std::time::Duration::from_secs(sec));
}

pub fn download_wallpapers(urls: Vec<String>, savepath: &str, bing: Option<bool>) {
    let bing = bing.unwrap_or(false);

    for url in urls{
            let default_bing_filename = "bing.jpg";
            let file_vec: Vec<&str>;

            if bing {
            file_vec = url.split("&rf=").collect();
            } else {
            file_vec = url.split("/").collect();
            }

            let filename = format!("{}/{}", savepath, file_vec[file_vec.len() - 1]);

            if filename.len() == 0 { panic!("Filename empty!") }
            else { 
                let res = download(url.as_str(), &filename);
                if res.is_err() {
                    panic!("cannot download url!")
                }
             }
        }
    }

fn download(url: &str, filename: &str) -> Result<(), Box<dyn std::error::Error>> {

    let filedest = PathBuf::from(filename);
    if filedest.exists(){
        return Ok(())
    }

    let mut response = reqwest::blocking::get(url).expect("Cannot download!");
    let mut out = File::create(filedest).expect("failed to create file");
    io::copy(&mut response, &mut out).expect("failed to copy content");
    Ok(())
}

pub fn random_n(len: usize) -> usize {
    let mut rng = rand::thread_rng();
    if len == 1 {return 0}
    rng.gen_range(0,len)
}

pub fn update_file_list(dirpath: &str) -> Vec<String> {

    let files = std::fs::read_dir(dirpath).unwrap();
    let mut wallpapers: Vec<String> = vec![];

    for file in files {
        let fp = file.unwrap().path().to_str().unwrap().to_string();

        if fp.ends_with("png"){ wallpapers.push(fp) }
        else if fp.ends_with("jpg") { wallpapers.push(fp) }
        else if fp.ends_with("jpeg") { wallpapers.push(fp) }
        else { continue }
    }

    return wallpapers
}