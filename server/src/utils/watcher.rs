use std::path::Path;

use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Result, Watcher};
use tokio::sync::mpsc;

pub fn watch_directory(path: &Path) -> Result<(RecommendedWatcher, mpsc::Receiver<Result<Event>>)> {
    let (tx, rx) = mpsc::channel(1);
    let mut watcher = RecommendedWatcher::new(
        move |res| {
            let _ = tx.blocking_send(res);
        },
        Config::default(),
    )?;

    watcher.watch(path, RecursiveMode::Recursive)?;

    Ok((watcher, rx))
}
