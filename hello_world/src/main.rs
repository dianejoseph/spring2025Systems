use std::process::Command;

fn executing_os_commands_linux() {
    let output = Command::new("ls")
        .arg("-l")
        let r = output.output().expect("Failed to execute command");

    //println!("Command output: {}", String::from_utf8_lossy(&output.stdout));
}