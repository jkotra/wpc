fn get_stdout(command: &str, args: &[&str]) {
    use std::process::Command;

    let output = Command::new(command).args(args).output();
    if output.is_ok() {
        ()
    } else {
        panic!("Command failed! (lib.rs)")
    }

    ()
}

pub fn run(command: &str, args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    get_stdout(command, args);
    Ok(())
}