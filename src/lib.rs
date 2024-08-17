#![deny(clippy::all)]

use std::process::Command;

#[macro_use]
extern crate napi_derive;
#[napi(object)]
pub struct ExcCommand {
  pub name: String,
  pub content: String,
}
#[napi]
pub fn run_commands(commands: Vec<ExcCommand>) {
  for command in commands {
    println!("{},{}", command.name, command.content);
    #[cfg(target_os = "macos")]
    {
      let _ = Command::new("osascript")
        .arg("-e")
        .arg(format!(
          "tell application \"Terminal\" to do script \"{}\"",
          command.content
        ))
        .spawn();
    }

    #[cfg(target_os = "windows")]
    {
      let _ = Command::new("cmd").arg("/C").arg(&command.content).spawn();
    }

    #[cfg(target_os = "linux")]
    {
      let _ = Command::new("sh").arg("-c").arg(&command.content).spawn();
    }
  }
}
