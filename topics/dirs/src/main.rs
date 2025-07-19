use dirs::*;

pub mod exercise;
use crate::exercise::testing;

fn main() {
    // Home dir — base of all user stuff
    println!("HOME        => {:?}", home_dir().unwrap());

    // Config dir — store settings, .toml/.json files
    println!("CONFIG      => {:?}", config_dir().unwrap());

    // Data dir — long-term storage (DBs, state files)
    println!("DATA        => {:?}", data_dir().unwrap());

    // Cache dir — temp crap that can be deleted
    println!("CACHE       => {:?}", cache_dir().unwrap());

    //  Downloads dir — user downloads (optional)
    println!("DOWNLOADS   => {:?}", download_dir().unwrap_or_default());

    // Desktop dir — user desktop (optional)
    println!("DESKTOP     => {:?}", desktop_dir().unwrap_or_default());

}

// USE CASES:
// home_dir()     => fallback or user paths
// config_dir()   => ~/.config/my_app/config.json
// data_dir()     => ~/.local/share/my_app/data.db
// cache_dir()    => ~/.cache/my_app/temp.json

// NOTES:
// - All return Option<PathBuf>. Use `.unwrap()` only if you're sure.
// - Cross-platform: works on Linux, Windows, macOS
// - dirs is frozen. Use `directories` crate if starting new lib work.


