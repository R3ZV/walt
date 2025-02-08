use std::fs;
use std::io;
use std::process;

pub fn read_wallpapers(directory_path: String) -> Result<Vec<String>, io::Error> {
    let wallpapers = fs::read_dir(directory_path)?
        .filter_map(|entry| {
            entry
                .ok()
                .and_then(|ent| ent.path().to_str().map(|str| str.to_string()))
        })
        .collect();

    Ok(wallpapers)
}

pub fn set_wallpaper(wallpaper_path: &str, platform: &str) {
    if platform == "wayland" {
        process::Command::new("swww")
            .args(["img", wallpaper_path, "--transition-step", "10"])
            .spawn()
            .expect("failed to set the new wallpaper");
    } else {
        process::Command::new("feh")
            .args(["--bg-scale", wallpaper_path])
            .spawn()
            .expect("failed to set the new wallpaper");
    }
}
