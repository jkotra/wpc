use std::process::Command;

#[cfg(target_os = "windows")]
use crate::windows::set_wallpaper_win;

pub fn change_wallpaper_gnome(file: &str){

    let wp = String::from("file://") + file;

    let out = Command::new("gsettings")
        .arg("set")
        .arg("org.gnome.desktop.background")
        .arg("picture-uri")
        .arg(&wp)
        .output();

    if out.is_err(){
        panic!("Error while changing wallpaper: {:?}", out.unwrap().status)
    }

}

#[cfg(target_os = "windows")]
pub fn change_wallpaper_windows(file: &str) {

        set_wallpaper_win(file);
}
