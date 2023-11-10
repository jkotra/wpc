use crate::misc;
use gio::traits::SettingsExt;
use gio::Settings;
use log::debug;
use log::info;
use std::path::PathBuf;

pub fn change_wallpaper_gnome(file: &str, theme: Option<String>) {
    let pb = PathBuf::from(file);
    if !pb.exists() {
        return;
    }

    let wp = String::from("file://") + file;
    let bg_settings = Settings::new("org.gnome.desktop.background");
    let if_settings = Settings::new("org.gnome.desktop.interface");

    match theme {
        Some(x) => if_settings.set_string("color-scheme", &x).unwrap(),
        None => (),
    }

    match bg_settings.set_string("picture-uri", wp.as_str()) {
        Ok(()) => (),
        Err(why) => debug!("{:?}", why),
    }

    match bg_settings.set_string("picture-uri-dark", wp.as_str()) {
        Ok(()) => (),
        Err(why) => debug!("picture-uri-dark error: {:?}", why),
    }
}

fn get_systemd_unit_path() -> std::path::PathBuf {
    let mut sysd_path = dirs::home_dir().unwrap();
    sysd_path.push(".config/systemd/user/");
    sysd_path
}

pub fn add_to_startup_gnome() -> Result<bool, String> {
    let mut sysd_path = get_systemd_unit_path();

    let curr_exe = std::env::current_exe().unwrap();

    let mut args: Vec<String> = misc::get_wpc_args();
    args.remove(0);
    let args = args.join(" ");

    info!(
        "target unit file path: {:?}/wpc.service",
        sysd_path.as_os_str()
    );

    let startup = format!(
        " \
     [Unit] \
    \nDescription=Wallpaper Changer \
    \nRequires=graphical-session.target \
    \n\n [Service] \
    \nEnvironment='RUST_LOG=info' \
    \nExecStart={exe} {args} \
    \nRestart=on-failure \
    \nRestartSec=30 \
    \n\n [Install] \
    \nWantedBy=default.target
    ",
        exe = curr_exe.to_str().unwrap(),
        args = args
    );

    info!("unit file: {}", startup);

    if sysd_path.exists() {
        match std::fs::remove_file(sysd_path.to_str().unwrap().to_string() + "wpc.service") {
            Ok(_) => info!("removed old wpc.service."),
            Err(e) => info!("{:#?}", e),
        }
    } else {
        std::fs::create_dir_all(sysd_path.as_path()).expect("cannot create recursive dirs.");
    }

    // add file to path.
    sysd_path.push("wpc.service");

    std::fs::write(sysd_path.as_path(), startup).expect("cannot write to unit file.");
    info!("wpc.service created!");

    let resp = std::process::Command::new("systemctl")
        .args(vec!["--user", "enable", "wpc"])
        .output()
        .expect("cannot enable unit.");

    match resp.status.code() {
        Some(code) => {
            if code == 0 {
                return Ok(true);
            }
        }
        None => return Err(String::from_utf8_lossy(&resp.stdout).to_string()),
    }

    return Ok(true);
}

pub fn rm_startup() -> bool {
    let sysd_path = get_systemd_unit_path();
    if sysd_path.exists() {
        let resp = std::process::Command::new("systemctl")
            .args(vec!["--user", "disable", "wpc"])
            .output()
            .expect("cannot disable unit.");
        match resp.status.code() {
            Some(code) => return code == 0,
            None => return false,
        }
    }
    false
}
