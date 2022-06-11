use gio::traits::SettingsExt;
use gio::{Settings};
use log::debug;
use log::{info};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::misc;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct GnomeVersion {
    platform: i32,
    minor: i32,
    micro: i32,
    distributor: String
}

fn get_gnome_version() -> GnomeVersion {
    let gv = serde_xml_rs::from_str(
        &std::fs::read_to_string("/usr/share/gnome/gnome-version.xml").unwrap()
    ).unwrap();
    return gv;
}

pub fn change_wallpaper_gnome(file: &str) {
    let pb = PathBuf::from(file);
    if !pb.exists() {
        return;
    }

    let wp = String::from("file://") + file;
    let bg_settings = Settings::new("org.gnome.desktop.background");

    match bg_settings.set_string("picture-uri", wp.as_str()) {
        Ok(()) => (),
        Err(why) => debug!("{:?}", why)
    }

    let version = get_gnome_version();

    if version.platform >= 42 {
        match bg_settings.set_string("picture-uri-dark", wp.as_str()) {
            Ok(()) => (),
            Err(why) => debug!("{:?}", why)
        }
    }
    
}

pub fn add_to_startup_gnome() -> Result<bool, Box<dyn std::error::Error>> {
    let mut args: Vec<String> = misc::get_wpc_args();
    args.remove(0);

    let curr_exe = std::env::current_exe().unwrap();
    let curr_exe = curr_exe.to_str().unwrap();
    let args = args.join(" ");

    let mut sysd_path = dirs::home_dir().unwrap();
    sysd_path.push(".config/systemd/user/");

    info!("{:?}/wpc.service", sysd_path.as_os_str());

    let startup = format!("

    [Unit]
    Description=Wallpaper Changer
    Requires=graphical-session.target

    [Service]
    Environment='RUST_LOG=info'
    ExecStart={exe} {args}
    Restart=always
    RestartSec=10

    [Install]
    WantedBy=default.target

    ", exe=curr_exe, args=args);

    info!("unit file: {}", startup);

    if sysd_path.exists(){
        match std::fs::remove_file(sysd_path.to_str().unwrap().to_string() + "wpc.service"){
            Ok(_) => info!("removed old wpc.service."),
            Err(e) => info!("{:#?}", e)
        }
    }
    else{
        std::fs::create_dir_all(sysd_path.as_path()).expect("cannot create recursive dirs.");
    }

    // add file to path.
    sysd_path.push("wpc.service");

    std::fs::write(sysd_path.as_path(), startup).expect("cannot write to unit file.");
    info!("wpc.service created!");

    /*
    let resp = std::process::Command::new("systemctl")
    .args(vec!["--user", "daemon-reload"])
    .output()
    .expect("cannot reload systemd daemon.");
    info!("{:#?}", resp);

    if resp.status.code().unwrap() == 0{
        info!("systemd daemon reloaded!");
    }
    */

    let resp = std::process::Command::new("systemctl")
    .args(vec!["--user", "enable", "wpc"])
    .output()
    .expect("cannot enable unit.");

    info!("{:#?}", resp);

    if resp.status.code().unwrap() == 0{
        info!("wpc.service enabled!");
        info!("wpc added to startup!");
    }

    return Ok(true);
}