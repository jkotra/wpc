use std::path::Path;
use std::io;
use std::fs::File;

use std::io::Write;

use std::path::PathBuf;

use dirs;

extern crate rand;
use rand::Rng;
use chrono;


pub fn print_debug_msg(content: &str) {
    println!("[DEBUG {:?}]: {}", chrono::offset::Local::now(), content)
}

pub fn wait(sec: u64) {
    std::thread::sleep(std::time::Duration::from_secs(sec));
}

pub fn download_wallpapers(urls: Vec<String>, savepath: &str, bing: Option<bool>) -> Vec<String> {
    let bing = bing.unwrap_or(false);
    let mut fileman: Vec<String> = vec![];

    for url in urls{
            let file_vec: Vec<&str>;

            if bing {
            file_vec = url.split("&rf=").collect();
            } else {
            file_vec = url.split("/").collect();
            }

            let filename = format!("{}/{}", savepath, file_vec[file_vec.len() - 1]);
            fileman.push(filename.clone());

            if filename.len() == 0 { panic!("Filename empty!") }
            else { 
                let res = download(url.as_str(), &filename);
                if res.is_err() {
                    panic!("cannot download url!")
                }
             }
        }

    return fileman;

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

// linux distro checks

// Patch Note - budgie-desktop added. based on gnome

pub fn is_linux_gnome_de() -> bool {
    let res = std::env::var("DESKTOP_SESSION").unwrap().to_string();
    if res == "gnome".to_string() { return true }
    if res == "gnome-xorg".to_string() { return true } //fedora
    if res == "budgie-desktop".to_string() { return true }
    return false;
}

pub fn is_linux_kde_de() -> bool {
    let res = std::env::var("DESKTOP_SESSION").unwrap().as_str().to_string();
    if res.contains("plasma") { return true }
    return false;
}

pub fn add_to_startup_gnome(wdir: String, interval: i32, update_interval: i32) -> bool{

    if !is_linux_gnome_de(){
        println!("Distro not supported!");
        return false;
    }

    let curr_exe = std::env::current_exe().unwrap();
    let curr_exe = curr_exe.to_str().unwrap();

    let home = dirs::home_dir().unwrap();
    let home = home.to_str().unwrap();

    let startup = format!("[Desktop Entry]
Type=Application
Name=WPC
Exec={} -d {} -l -i {} -u {}
Icon=
Comment=
X-GNOME-Autostart-enabled=true\n",
                          curr_exe, wdir, interval, update_interval);

    let startup_path = format!("{}/.config/autostart/wpc.desktop", home.to_owned());

let mut f = File::create(&startup_path).expect("cannot create startup file!");
    let res = f.write_all(startup.as_bytes());

    if res.is_err() != true && Path::new(&startup_path).exists() {
        println!("Added to startup.");
    }
    return true;

}


