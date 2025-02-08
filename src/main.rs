mod setters;
mod tui;

use crate::setters::set_random_wallpaper;
use color_eyre::Result;
use homedir::my_home;

fn print_help() {
    println!("USAGE");
    println!("    walt [OPTION]");

    println!("OPTIONS:");
    println!("    -h, --help            Prints this message.");
    println!("    -nt, --no-tui         Sets a random wallpaper without running the TUI.");
}

fn main() -> Result<()> {
    let args = std::env::args();
    if args.len() > 2 {
        println!("Invalid number of arguments!");
        println!("Use: walt --help");
        return Ok(());
    }

    let mut run_tui = true;
    let command = args.skip(1).next();
    if let Some(cmd) = command {
        match cmd.as_str() {
            "--no-tui" | "-nt" => run_tui = false,
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

    let platform = "X11";
    // TODO: check for platform
    // TODO: check based on platform if
    // setter exists

    let user_home = my_home()?.expect("Couldn't get user home directory");
    let wallpapers_path = format!("{}/Pictures/Wallpapers", user_home.display());

    if run_tui {
        color_eyre::install()?;

        let terminal = ratatui::init();

        let result = tui::App::new(&wallpapers_path, &platform).run(terminal);
        ratatui::restore();

        return Ok(result?);
    }

    set_random_wallpaper(&wallpapers_path, &platform)?;

    Ok(())
}
