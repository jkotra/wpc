use dirs;
use enquote;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use Result;
use crate::lib::run;


//Taken from: reujab/wallpaper.rs


/// Sets the wallpaper for KDE.
pub fn set(path: &str) {
    run(
        "qdbus",
        &[
            "org.kde.plasmashell",
            "/PlasmaShell",
            "org.kde.PlasmaShell.evaluateScript",
            &format!(
                r#"
const monitors = desktops()
for (var i = 0; i < monitors.length; i++) {{
    monitors[i].currentConfigGroup = ["Wallpaper"]
    monitors[i].writeConfig("Image", {})
}}"#,
                enquote::enquote('"', &format!("file://{}", path)),
            ),
        ],
    );
}