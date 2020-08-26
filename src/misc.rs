use std::io;
use std::fs::File;
use std::path::PathBuf;

extern crate rand;
use rand::Rng;


pub fn print_debug_msg(content: &str) {
    println!("[DEBUG {:?}]: {}", chrono::offset::Local::now(), content)
}

pub fn wait(sec: u64) {
    std::thread::sleep(std::time::Duration::from_secs(sec));
}

pub fn download_wallpapers(urls: Vec<String>, savepath: &str, bing: bool) -> Vec<String> {
    let mut remote_files: Vec<String> = vec![];

    for url in urls{
            let file_vec: Vec<&str>;

            if bing {
            file_vec = url.split("&rf=").collect();
            } else {
            file_vec = url.split("/").collect();
            }

            let mut filename = format!("{}/{}", savepath, file_vec[file_vec.len() - 1]);

            if bing {
                filename = format!("{}/{}", savepath, "bing_wpod.jpg");
            }
            remote_files.push(filename.clone());

            if filename.len() == 0 { panic!("Filename empty!") }
            else { 
                let res = download(url.as_str(), &filename);
                if res.is_err() {
                    panic!("cannot download url!")
                }
             }

    }

    return remote_files;
}

pub fn download(url: &str, filename: &str) -> Result<(), Box<dyn std::error::Error>> {

    let filedest = PathBuf::from(filename);
    if filedest.exists(){
        return Ok(())
    }

    let mut response = reqwest::blocking::get(url).expect("Cannot download!");
    let mut out = File::create(filedest).expect("failed to create file");
    io::copy(&mut response, &mut out).expect("failed to copy content");
    Ok(())
}

pub fn random_n(len_max: usize) -> usize {
    let mut rng = rand::thread_rng();
    if len_max == 1 {return 0}
    rng.gen_range(0,len_max)
}

pub fn update_file_list(dirpath: &str, maxage: i64) -> Vec<String> {

    let files = std::fs::read_dir(dirpath).unwrap();
    let mut wallpapers: Vec<String> = vec![];

    for file in files {
        let fp = file.unwrap().path().to_str().unwrap().to_string();

        //compute age diff and continue if older then max age!
        if maxage != -1{

            //current time as timestamp
            let maxage_time = chrono::Local::now().timestamp() - i64::from(maxage * 60 * 60);

            //get created date and convert to timestamp.
            let f_ct = std::fs::metadata(&fp).unwrap().created().unwrap().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();

            if maxage_time as u64 > f_ct{
                continue;
            }

        }

        if fp.ends_with("png"){ wallpapers.push(fp) }
        else if fp.ends_with("jpg") { wallpapers.push(fp) }
        else if fp.ends_with("jpeg") { wallpapers.push(fp) }
        else { continue }
    }

    return wallpapers
}


pub fn is_linux_gnome_de() -> bool {
    let res = std::env::var("DESKTOP_SESSION").unwrap().to_string();
    if res == "gnome".to_string() { return true }
    if res == "gnome-xorg".to_string() { return true } //fedora
    if res == "budgie-desktop".to_string() { return true } //budgie
    return false;
}
