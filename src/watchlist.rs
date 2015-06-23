extern crate notify;

use notify::{RecommendedWatcher, Error, Watcher};
use std::sync::mpsc::channel;

use std;
use std::path::Path;
use std::string::String;
use std::fs;
use std::fs::Metadata;

pub struct WatchList {
    dirs: Vec<String>,
    files: Vec<String>,
    lastChecked: u64,
    watcher: RecommendedWatcher,
}

impl WatchList {
    pub fn new() -> (WatchList, Error) {
        WatchList { dirs: Vec::<String>::new(), files: Vec::<String>::new(), lastChecked: 0 }

        let (tx, rx) = channel();
        let mut w: Result<RecommendedWatcher, Error> = Watcher::new(tx);

        match w {

            Ok(mut watcher) =>  (WatchList { dirs: Vec::<String>::new(), files: Vec::<String>::new(), lastChecked: 0 }, null),
            Err(e) => (WatchList {}, e)
        }
    }

    pub fn watch_dir(&mut self, dir: String) {
        self.dirs.push(dir);
    }

    pub fn check(&mut self) -> bool {
        for i in 0..self.dirs.len() {
            let attr = match std::fs::metadata(&self.dirs[i]) {
                Ok(data) => data,
                Err(error) => {
                    return false;
                },
            };

            if attr.modified() > self.lastChecked {
                return true;
            }

        }

        return false;
    }
}

