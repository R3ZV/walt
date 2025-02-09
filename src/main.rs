mod setters;
mod tui;

use std::process::{Command, ExitStatus, Stdio};

use crate::setters::{set_random_wallpaper, Platform};
use color_eyre::Result;
use homedir::my_home;

static VERSION: &str = "0.2.1";

fn print_help() {
    println!("USAGE");
    println!("    walt [OPTION]");

    println!("OPTIONS:");
    println!("    -v, --version            Prints version information.");
    println!("    -h, --help            Prints this message.");
    println!("    -nt, --no-tui         Sets a random wallpaper without running the TUI.");
}

fn check_installed(setter: &str) -> std::io::Result<ExitStatus> {
    Command::new(setter)
        .stdout(Stdio::null())
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .arg("--help")
        .spawn()?
        .wait()
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut args = std::env::args();
    if args.len() > 2 {
        println!("Invalid number of arguments!");
        println!("Use: walt --help");
        return Ok(());
    }

    let mut run_tui = true;
    let command = args.nth(1);
    if let Some(cmd) = command {
        match cmd.as_str() {
            "--no-tui" | "-nt" => run_tui = false,
            "--version" | "-v" => {
                println!("Walt version: {}", VERSION);
                return Ok(());
            }
            "--help" | "-h" => {
                print_help();
                return Ok(());
            }
            _ => {
                println!("Invalid option!");
                println!("Use: walt --help");
                return Ok(());
            }
        }
    }

    let platform = match std::env::var("XDG_SESSION_TYPE") {
        Ok(session_server) => match session_server.as_str() {
            "x11" => Platform::X11,
            _ => Platform::Wayland,
        },
        Err(_) => Platform::X11,
    };

    let setter = match platform {
        Platform::X11 => "feh",
        Platform::Wayland => "swww",
    };

    match check_installed(setter) {
        Ok(_) => {}
        Err(_) => {
            panic!(
                "{} not found! Make sure you have it installed and it is in your PATH!",
                &setter
            );
        }
    }

    let user_home = my_home()?.expect("Couldn't get user home directory");
    let wallpapers_path = format!("{}/Pictures/Wallpapers", user_home.display());

    if run_tui {
        let result = tui::App::new(&wallpapers_path, platform);

        match result {
            Ok(_) => {}
            Err(err) => match err.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("[ERROR]: '{}' doesn't exist!", &wallpapers_path)
                }
                _ => panic!(
                    "[ERROR]: Couldn't open '{}' due to: {}",
                    &wallpapers_path, err
                ),
            },
        }

        let terminal = ratatui::init();
        let result = result.unwrap().run(terminal);
        ratatui::restore();

        return Ok(result?);
    }

    match set_random_wallpaper(&wallpapers_path, platform) {
        Ok(_) => {}
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => {
                panic!("[ERROR]: '{}' doesn't exist!", &wallpapers_path)
            }
            _ => panic!("[ERROR]: Couldn't set wallpaper due to: {}", err),
        },
    }

    Ok(())
}
