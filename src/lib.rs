use std::io;
use std::process::Command;

pub fn reboot() -> Result<(), io::Error> {
    let _output = prepare_command().output()?;
    Ok(())
}

#[cfg(target_os = "windows")]
fn prepare_command() -> Command {
    let mut command = Command::new("shutdown");
    command.arg("/r");
    command.arg("/f");
    command.arg("/t");
    command.arg("0");
    command
}

#[cfg(not(target_os = "windows"))]
fn prepare_command() -> Command {
    Command::new("reboot")
}
