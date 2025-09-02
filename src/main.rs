use notify::{Event, RecursiveMode, Result, Watcher};
use std::{env, path::Path, sync::mpsc};
use tracing::info;
use tracing_subscriber::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel::<Result<Event>>();

    let mut watcher = notify::recommended_watcher(tx)?;

    let watch_path = env::var("WATCH_PATH").unwrap_or_default();
    println!("watching {}", watch_path);

    //logger
    let log_dir_path = env::var("Z_LOG_DIR_PATH").unwrap_or_default();
    let file_appender = tracing_appender::rolling::never(log_dir_path, "files_log.txt");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(std::io::stdout)
                .with_filter(tracing_subscriber::filter::LevelFilter::INFO),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(non_blocking)
                .with_ansi(false)
                .with_filter(tracing_subscriber::filter::LevelFilter::INFO),
        )
        .init();

    watcher.watch(Path::new(&watch_path), RecursiveMode::Recursive)?;
    // Block forever, printing out events as they come in
    for res in rx {
        match res {
            Ok(event) => match event.kind {
                notify::EventKind::Create(_) => {
                    info!(name: "Created", "{:?}", event.paths);
                }
                _ => {}
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}
