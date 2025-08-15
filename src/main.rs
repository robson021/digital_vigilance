use crate::config_holder::{SharedConfig, TaskUptime, VigilanceTaskMetadata};
use crate::helpers::{FromMin, log_debug};
use crate::menu_builder::build_menu;
use crate::popup_notification::{show_message, show_time_remaining_notification};
use std::process::exit;
use std::sync::Arc;
use std::time::Duration;
use tokio::select;
use tokio::sync::broadcast;
use tokio::sync::broadcast::Sender;

mod config_holder;
mod helpers;
mod menu_builder;
mod mouse_handler;
mod popup_notification;

const APP_NAME: &str = "Digital Vigilance";
const DEFAULT_UPTIME_SEC: TaskUptime = TaskUptime::Timed(Duration::from_secs(60 * 60));

#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() {
    let config = VigilanceTaskMetadata::new(DEFAULT_UPTIME_SEC);
    let cloned_config = Arc::clone(&config);

    let (tx, _) = broadcast::channel::<()>(1);
    let tx_cloned = tx.clone();

    tokio::spawn(async move {
        move_with_interval(cloned_config, tx_cloned).await;
        show_message("Done!");
        tokio::time::sleep(Duration::from_secs(2)).await;
        exit(0);
    });
    build_menu(config, tx);
}

async fn move_with_interval(cfg: SharedConfig, tx: Sender<()>) {
    let mut rx = tx.subscribe();
    'task: loop {
        let uptime = {
            let mut guard = cfg.lock().await;
            guard.set_start_time_to_now();
            guard.uptime
        };
        let minutes = match uptime {
            TaskUptime::Infinite => u64::MAX,
            TaskUptime::Timed(duration) => duration.as_minutes(),
        };
        log_debug(&format!("New task with {minutes} minutes duration"));
        show_time_remaining_notification(&uptime);

        let mut gracefully_shutdown = true;

        for _ in 0..minutes {
            select! {
                _ = rx.recv() => {
                    log_debug("Signal received");
                    gracefully_shutdown = false;
                    break;
                }
                _ = tokio::time::sleep(Duration::from_min(1)) => mouse_handler::move_silently()
            }
        }
        match gracefully_shutdown {
            true => break 'task,
            false => log_debug("Broken by signal"),
        }
    }
}
