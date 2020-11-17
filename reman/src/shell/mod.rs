use std::{
    io::{BufRead, BufReader, Error, ErrorKind, Result},
    path::Path,
    process::{Command, Stdio},
  };

use ansi_term::Colour::{ Yellow, Cyan };

pub fn execute(cmd: &str, args: &[ &str ], verbose: bool) {

    execute_in(cmd, args, &None, verbose);
}

pub fn execute_in(cmd: &str, args: &[ &str], current_directory: &Option<&Path>, verbose: bool) {

    let mut command = execute_command(&cmd, &args, &current_directory, verbose);
    let _ = output(&mut command);
}

pub fn execute_in_with_output(instruction: &str, arguments: &[&str], current_directory: &Option<&Path>, verbose: bool) -> String {

    let mut command = execute_command(&instruction, &arguments, &current_directory, verbose);
    let output = command.output().expect("failed to execute command");
    String::from_utf8_lossy(&output.stdout).to_string()
}

// ------------------------------------------------------------------------------------------------------------

fn execute_command(cmd: &str, args: &[ &str], current_directory: &Option<&Path>, verbose: bool) -> Command {

    let mut command = Command::new(cmd);
    command.args(args);
    if let Some(directory) = current_directory {
      command.current_dir(directory);
    }
    if verbose {
      let command_output = format!("{:?}", command);
      print!("{}", Cyan.paint("\nCommand: "));
      println!("{}\n", Yellow.paint(command_output));
    }
    
    command
}

fn output(command: &mut Command) -> Result<()> {
    let output = command
      .stdout(Stdio::piped())
      .spawn()?
      .stdout
      .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))?;
    let reader = BufReader::new(output);
    reader
      .lines()
      .filter_map(|line| line.ok())
      .for_each(|line| println!("{}", line));
    Ok(())
  }
