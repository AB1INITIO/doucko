//! Polls the site's sources for changes, as std has no file-system
//! notifications, and hands what changed to [`Site::update`]. A poll compares
//! each file's modification time and size, so it never reads a file.

use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::thread;
use std::time::{Duration, Instant, SystemTime};

use crate::site::{self, Site};

const POLL: Duration = Duration::from_millis(250);

/// Each source file's modification time and size.
type Snapshot = BTreeMap<PathBuf, (SystemTime, u64)>;

pub struct Watcher {
    last: Snapshot,
}

impl Watcher {
    /// Starts from the sources' current state, so that edits made from now on
    /// (say, during the initial build) are picked up.
    pub fn new() -> Self {
        Watcher {
            last: snapshot().unwrap_or_default(),
        }
    }

    pub fn run(mut self, site: &Site) -> ! {
        println!("Watching for changes (Ctrl-C to stop)...");
        loop {
            thread::sleep(POLL);
            let Ok(now) = snapshot() else {
                continue; // say, a directory vanished mid-scan: try again
            };
            let changed = diff(&self.last, &now);
            self.last = now;
            if changed.is_empty() {
                continue;
            }
            let names: Vec<_> = changed
                .iter()
                .map(|path| format!("\"{}\"", path.display()))
                .collect();
            println!("changed: {}", names.join(" "));
            let start = Instant::now();
            match site.update(&changed) {
                Ok(()) => println!("updated in {} ms.", start.elapsed().as_millis()),
                Err(e) => eprintln!("update FAILED: {e}; the rest of the site is still served."),
            }
        }
    }
}

fn snapshot() -> io::Result<Snapshot> {
    Ok(site::sources()?
        .into_iter()
        .filter_map(|path| {
            let meta = fs::metadata(&path).ok()?; // gone: a deletion
            Some((path, (meta.modified().ok()?, meta.len())))
        })
        .collect())
}

/// The paths added, modified or deleted between two snapshots, sorted.
fn diff(old: &Snapshot, new: &Snapshot) -> Vec<PathBuf> {
    let modified = new
        .iter()
        .filter(|&(path, stamp)| old.get(path) != Some(stamp))
        .map(|(path, _)| path);
    let deleted = old.keys().filter(|path| !new.contains_key(*path));
    let mut paths: Vec<_> = modified.chain(deleted).cloned().collect();
    paths.sort();
    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diffs_snapshots() {
        let snapshot = |files: &[(&str, u64)]| -> Snapshot {
            files
                .iter()
                .map(|&(path, size)| (path.into(), (SystemTime::UNIX_EPOCH, size)))
                .collect()
        };
        let old = snapshot(&[("kept", 1), ("edited", 1), ("deleted", 1)]);
        let new = snapshot(&[("kept", 1), ("edited", 2), ("added", 1)]);
        assert_eq!(
            diff(&old, &new),
            ["added", "deleted", "edited"].map(PathBuf::from)
        );
    }
}
