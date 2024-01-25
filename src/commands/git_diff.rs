use chrono::Local;
use colored::Colorize;
use std::{
    fs::File,
    io::{self, Write},
    path::Path,
    process::Command,
};

use crate::args::{CommitOptions, PullOptions, PushOptions};
use crate::config::Config;

fn check_if_git_repo(path: &Path) -> bool {
    let output = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .current_dir(path)
        .output();
    match output {
        Ok(output) => return String::from_utf8_lossy(&output.stdout).eq(&String::from("true\n")),
        Err(_) => return false,
    }
}

pub fn commit(config: Config, arguments: CommitOptions, mut _log_file: File) -> io::Result<()> {
    // Get path from the file_path str
    let config_path = expanduser::expanduser(&config.path)?;
    let path = match Path::new(&config_path).parent() {
        Some(path) => path,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error getting the path for the backup"),
            ))
        }
    };

    // Check if there is a git repository
    let repo = check_if_git_repo(path);
    if repo == true {
        // Creates the commit
        let _ = Command::new("git")
            .args(["add", "."])
            .current_dir(path)
            .output();
        let output = Command::new("git")
            .arg("commit")
            .args(arguments.commit_options)
            .current_dir(path)
            .status()
            .unwrap();
        if output.success() {
            println!("{} Created commit successfully", "[OK]".bold().green());
        } else {
            println!("{} Couldn't create commit", "[ERROR]".bold().red());
            let message = format!(
                "[{}][COMMIT] <- {:?}",
                Local::now().format("%d-%m-%Y %H:%M:%S"),
                output
            );
            let _ = _log_file.write_all(message.as_bytes());
        }
    } else {
        println!("{} Directory is not a git repo", "[ERROR]".bold().red());
    }

    Ok(())
}

pub fn push(config: Config, arguments: PushOptions, mut _log_file: File) -> io::Result<()> {
    // Get path from the file_path str
    let config_path = expanduser::expanduser(&config.path)?;
    let path = match Path::new(&config_path).parent() {
        Some(path) => path,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error getting the path for the backup"),
            ))
        }
    };

    // Check if there is a git repository
    let repo = check_if_git_repo(path);
    if repo == true {
        // Push using git
        let output = Command::new("git")
            .arg("push")
            .args(arguments.commit_options)
            .current_dir(path)
            .status()
            .unwrap();
        if output.success() {
            println!("{} Pushed successfully", "[OK]".bold().green());
        } else {
            println!("{} Couldn't pushed successfully", "[ERROR]".bold().red());
            let message = format!(
                "[{}][PUSH] <- {:?}",
                Local::now().format("%d-%m-%Y %H:%M:%S"),
                output
            );
            let _ = _log_file.write_all(message.as_bytes());
        }
    } else {
        println!("{} Directory is not a git repo", "[ERROR]".bold().red());
    }

    Ok(())
}

pub fn pull(config: Config, arguments: PullOptions, mut _log_file: File) -> io::Result<()> {
    // Get path from the file_path str
    let config_path = expanduser::expanduser(&config.path)?;
    let path = match Path::new(&config_path).parent() {
        Some(path) => path,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error getting the path for the backup"),
            ))
        }
    };

    // Check if there is a git repository
    let repo = check_if_git_repo(path);
    if repo == true {
        // Pull using git
        let output = Command::new("git")
            .arg("pull")
            .args(arguments.commit_options)
            .current_dir(path)
            .status()
            .unwrap();
        if output.success() {
            println!("{} Pulled successfully", "[OK]".bold().green());
        } else {
            println!("{} Couldn't pulled successfully", "[ERROR]".bold().red());
            let message = format!(
                "[{}][PULL] <- {:?}",
                Local::now().format("%d-%m-%Y %H:%M:%S"),
                output
            );
            let _ = _log_file.write_all(message.as_bytes());
        }
    } else {
        println!("{} Directory is not a git repo", "[ERROR]".bold().red());
    }

    Ok(())
}

pub fn diff(config: Config, mut _log_file: File) -> io::Result<()> {
    println!("Diff");
    Ok(())
}
