use crate::{settings::ThemeOptions, misc};

use log::{debug, info};

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
mod gnome;

pub fn change_wallpaper(uri: &str, theme_options: ThemeOptions){

    #[cfg(target_os = "windows")]
    windows::set_wallpaper_win(uri);


    #[cfg(target_os = "linux")]
    let mut theme: Option<String> = Option::None;
    let b_score = misc::brighness_score(uri);
    info!("brightness_score = {}", b_score);
    if theme_options.set_theme{
        if misc::brighness_score(uri) as f32 >= theme_options.theme_th{
                if theme_options.theme_dark_only{
                    return
                }
                theme = Some("prefer-light".to_string())
            }
            else{
                if theme_options.theme_light_only{
                    return
                }
                theme = Some("prefer-dark".to_string())
        }
    }

    gnome::change_wallpaper_gnome(uri, theme);

}

pub fn add_to_startup(){


    #[cfg(target_os = "windows")]
    windows::add_to_startup_reg();

    #[cfg(target_os = "linux")]
    gnome::add_to_startup_gnome().expect("Error while adding to startup.");

}