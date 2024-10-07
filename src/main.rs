use std::path::PathBuf;

use inotify::{Inotify, WatchMask};

const BONK_SPOTIFY: &str = "explicit_filter.restore";

fn main() {
    let mut inotify = Inotify::init().expect("Error initializing inotify instance");

    let home = &std::env::var("HOME").expect("Please set $HOME to use this program");
    inotify
        .watches()
        .add(home, WatchMask::CREATE)
        .expect("Failed to add directory watch");

    let bad_file = &{
        let mut path = PathBuf::from(home);
        path.push(BONK_SPOTIFY);
        path
    };

    // try and remove it if it already exists
    std::fs::remove_file(bad_file).ok();

    let mut buf = [0; 1024];
    loop {
        let events = inotify
            .read_events_blocking(&mut buf)
            .expect("Error while reading events");

        for event in events {
            let Some(name) = event.name else { continue };

            // try and remove it got created lets remove it
            if name == BONK_SPOTIFY {
                if let Some(error) = std::fs::remove_file(bad_file).err() {
                    eprintln!("Failed to remove file: {error:?}");
                }
            }
        }
    }
}
