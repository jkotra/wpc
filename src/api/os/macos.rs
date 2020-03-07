use Result;
use crate::lib::run;

//Taken from: reujab/wallpaper.rs

pub fn set_from_path(path: &str){
    run(
        "osascript",
        &[
            "-e",
            &format!(
                r#"tell application "Finder" to set desktop picture to POSIX file {}"#,
                enquote::enquote('"', path),
            ),
        ],
    );
}