use std::process::Command;


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