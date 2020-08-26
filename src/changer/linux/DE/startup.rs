use std::path::Path;
use std::fs::File;
use std::io::Write;


pub fn add_to_startup_gnome() -> bool{

    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0); //remove program name

    let args = args.join(" ");

    let mut args = args.replace("--debug", "");
    args = args.replace("-D", "");
    args = args.replace("--startup", "");

    println!("program args: {}", args);


    let curr_exe = std::env::current_exe().unwrap();
    let curr_exe = curr_exe.to_str().unwrap();

    let home = dirs::home_dir().unwrap();
    let home = home.to_str().unwrap();

    let startup = format!("\n
    [Desktop Entry]
    Type=Application
    Name=WPC
    Exec={} {}
    Icon=
    Comment=
    X-GNOME-Autostart-enabled=true\n", curr_exe, args);
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
