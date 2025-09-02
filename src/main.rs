use notify::{Event, RecursiveMode, Result, Watcher};
use std::{env, path::Path, sync::mpsc};

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel::<Result<Event>>();

    let mut watcher = notify::recommended_watcher(tx)?;

    let watch_path = env::var("WATCH_PATH").unwrap_or_default();
    println!("watching {}", watch_path);

    watcher.watch(Path::new(&watch_path), RecursiveMode::Recursive)?;
    // Block forever, printing out events as they come in
    for res in rx {
        match res {
            Ok(event) => println!("event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}
