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


pub fn set_wallpaper_win(path: &str) {
    let path = OsStr::new(path)
                .encode_wide()
                // append null byte
                .chain(iter::once(0))
                .collect::<Vec<u16>>();
            unsafe {
                let successful = SystemParametersInfoW(
                    SPI_SETDESKWALLPAPER,
                    0,
                    path.as_ptr() as *mut c_void,
                    SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
                );

                if successful != 1{
                    panic!("Error: Cannot set windows wallpaper!")
                }
            }
}

extern crate winreg;
use winreg::enums::*;


pub fn add_to_startup_reg() {
    println!("Trying to add WPC to startup...");

    let hkcu = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
    let subkey = hkcu.open_subkey_with_flags(r#"Software\Microsoft\Windows\CurrentVersion\Run"#,
                                    KEY_WRITE).expect("Failed to open subkey");
    
    let mut cmd = misc::get_wpc_args();
    cmd.push(String::from("--background"));
    let cmd = cmd.join(" ");                       

    subkey.set_value("WPC", &cmd).expect("cannot add value to reg!");
    println!("WPC added to startup!");

}