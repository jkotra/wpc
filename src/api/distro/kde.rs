use enquote;
use crate::lib::run;

//Taken from: reujab/wallpaper.rs


/// Sets the wallpaper for KDE.
pub fn set(path: &str) {
    let res = run(
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

    if res.is_err(){
        panic!("cannot set KDE wallpaper!");
    }

}