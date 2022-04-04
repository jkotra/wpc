use gio::{Settings, SettingsExt};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use crate::misc;


pub fn change_wallpaper_gnome(file: &str) {
    let pb = PathBuf::from(file);
    if !pb.exists() {
        return;
    }

    let wp = String::from("file://") + file;
    let bg_settings = Settings::new("org.gnome.desktop.background");
    let _ = bg_settings.set_string("picture-uri", wp.as_str());
    Settings::sync();
}

pub fn add_to_startup_gnome() -> bool {
    let mut args: Vec<String> = misc::get_wpc_args();
    args.remove(0);

    let curr_exe = std::env::current_exe().unwrap();
    let curr_exe = curr_exe.to_str().unwrap();

    let args = args.join(" ");

    let home = dirs::home_dir().unwrap();
    let home = home.to_str().unwrap();

    let startup = format!("\n[Desktop Entry]\nType=Application\nName=WPC\nExec={exe} {args}\nIcon=\nComment=\nX-GNOME-Autostart-enabled=true\n", exe=curr_exe, args=args);

    let startup_path = format!("{}/.config/autostart/wpc.desktop", home.to_owned());

    let mut f = File::create(&startup_path).expect("cannot create startup file!");
    let res = f.write_all(startup.as_bytes());

    if res.is_err() != true && Path::new(&startup_path).exists() {
        println!("Added to startup: {}", startup_path);
        return true;
    } else {
        return false;
    }
}
