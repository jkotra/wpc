use std::path::Path;
use std::fs::File;
use std::io::Write;

#[path = "../../../misc.rs"]
#[allow(unused)]
mod misc;

pub fn add_to_startup_gnome() -> bool{

    let mut args: Vec<String> = misc::get_wpc_args();
    args.remove(0);

    let curr_exe = std::env::current_exe().unwrap();
    let curr_exe = curr_exe.to_str().unwrap();

    let args = args.join(" ");

    let home = dirs::home_dir().unwrap();
    let home = home.to_str().unwrap();

    let startup = format!("\n[Desktop Entry]\nType=Application\nName=WPC\nExec={exe} {args}\nIcon=\nComment=\nX-GNOME-Autostart-enabled=true\n", exe=curr_exe, args=args);

    let startup_path = format!("{}/.config/autostart/wpc.desktop", home.to_owned());

    let mut f = File::create(&startup_path).expect("cannot create startup file!");
    let res = f.write_all(startup.as_bytes());

    if res.is_err() != true && Path::new(&startup_path).exists() {
        println!("Added to startup: {}", startup_path);
        return true;
    }
    else{
        return false;
    }
}
