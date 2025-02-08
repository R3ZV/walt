mod compositor;
mod tui;

use color_eyre::Result;
use homedir::my_home;

fn main() -> Result<()> {
    color_eyre::install()?;

    let terminal = ratatui::init();
    let user_home = my_home()?.expect("Couldn't get user home directory");
    let wallpapers_path = format!("{}/Pictures/Wallpapers", user_home.display());

    let result = tui::App::new(wallpapers_path).run(terminal);
    ratatui::restore();
    Ok(result?)
}
