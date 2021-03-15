
use gio::{Settings, SettingsExt};
use std::path::PathBuf;

pub fn change_wallpaper_gnome(file: &str){

    let pb = PathBuf::from(file);
    if !pb.exists(){
        return
    }
    
    let wp = String::from("file://") + file;
    let bg_settings = Settings::new("org.gnome.desktop.background");
    let _ = bg_settings.set_string("picture-uri", wp.as_str());
    Settings::sync();
}