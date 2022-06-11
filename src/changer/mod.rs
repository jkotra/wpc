#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
mod gnome;

pub fn change_wallpaper(uri: &str){

    #[cfg(target_os = "windows")]
    windows::set_wallpaper_win(uri);


    #[cfg(target_os = "linux")]
    gnome::change_wallpaper_gnome(uri);

}

pub fn add_to_startup(){


    #[cfg(target_os = "windows")]
    windows::add_to_startup_reg();

    #[cfg(target_os = "linux")]
    gnome::add_to_startup_gnome().expect("Error while adding to startup.");

}