// for windows API
// Windows functions

use std::ffi::OsStr;
use std::iter;
use std::os::raw::c_void;
use std::os::windows::ffi::OsStrExt;
use winapi::um::winuser::SystemParametersInfoW;
use winapi::um::winuser::SPIF_SENDCHANGE;
use winapi::um::winuser::SPIF_UPDATEINIFILE;
use winapi::um::winuser::SPI_SETDESKWALLPAPER;
extern crate winreg;
use crate::misc;
use winreg::enums::*;

use log::info;

pub fn set_wallpaper_win(path: &str, theme: Option<String>) {
    let path = OsStr::new(path)
        .encode_wide()
        // append null byte
        .chain(iter::once(0))
        .collect::<Vec<u16>>();
    match theme {
        Some(theme) => {
            info!("theme = {}", theme);
            if theme == "prefer-dark" {
                use_light_theme_reg(0)
            } else {
                use_light_theme_reg(1)
            }
        }
        None => (),
    }
    unsafe {
        let successful = SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            path.as_ptr() as *mut c_void,
            SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
        );

        if successful != 1 {
            panic!("Error: Cannot set windows wallpaper!")
        }
    }
}

pub fn add_to_startup_reg() {
    println!("Trying to add WPC to startup...");

    let hklu = winreg::RegKey::predef(HKEY_CURRENT_USER);
    let subkey = hklu
        .open_subkey_with_flags(
            r#"Software\Microsoft\Windows\CurrentVersion\Run"#,
            KEY_WRITE,
        )
        .expect("Failed to open subkey");

    let mut cmd = misc::get_wpc_args();
    cmd.push(String::from("--background"));
    let cmd = cmd.join(" ");

    subkey
        .set_value("WPC", &cmd)
        .expect("cannot add value to reg!");
    println!("WPC added to startup!");
}

pub fn rm_startup_reg() {
    println!("Removing WPC from startup...");

    let subkey = winreg::RegKey::predef(HKEY_CURRENT_USER)
        .open_subkey_with_flags(
            r#"Software\Microsoft\Windows\CurrentVersion\Run"#,
            KEY_ALL_ACCESS,
        )
        .expect("Failed to open subkey");

    match subkey.delete_value("WPC") {
        Ok(_) => info!("WPC removed from startup."),
        Err(err) => info!("#{:?}", err),
    }
}

pub fn use_light_theme_reg(value: u32) {
    let hkcu = winreg::RegKey::predef(HKEY_CURRENT_USER);
    let subkey = hkcu
        .open_subkey_with_flags(
            r#"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize"#,
            KEY_SET_VALUE | KEY_READ,
        )
        .expect("Failed to open subkey");

    /*
    let cvalue: u32 = subkey.get_value("AppsUseLightTheme").unwrap();
    println!("{:?}", cvalue);
    */

    subkey
        .set_value("AppsUseLightTheme", &value)
        .expect("cannot set reg value!");
}
