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