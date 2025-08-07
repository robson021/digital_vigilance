use crate::menu_builder::build_menu;
use crate::popup_notification::show_notification;
use crate::refresh_holder::{ConfigHolder, SharedConfig};
use std::sync::Arc;
use std::time::Duration;
use tokio::select;
use tokio::sync::broadcast;
use tokio::sync::broadcast::Sender;

mod menu_builder;
mod mouse_handler;
mod popup_notification;
mod refresh_holder;

const APP_NAME: &str = "Digital Vigilance";
const DEFAULT_UPTIME_SEC: u64 = 60 * 60;

#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() {
    let config = ConfigHolder::new(DEFAULT_UPTIME_SEC);
    let cloned_config = Arc::clone(&config);

    let (tx, _) = broadcast::channel::<()>(1);

    let tx_cloned = tx.clone();

    tokio::spawn(async move {
        move_with_interval(cloned_config, tx_cloned).await;
    });
    show_notification(DEFAULT_UPTIME_SEC / 60);

    build_menu(&config, tx);
}

async fn move_with_interval(cfg: SharedConfig, tx: Sender<()>) {
    let mut rx = tx.subscribe();
    let duration = cfg.lock().await.uptime.as_secs();
    let mut countdown = duration / 60;
    log_debug(&format!("New task with {countdown} iterations spawned"));

    loop {
        select! {
            _ = rx.recv() => {
                log_debug("Ending current task");
                break;
            }
            _ = tokio::time::sleep(Duration::from_secs(60)) => {
                if countdown > 0 {
                    countdown -= 1;
                    mouse_handler::move_silently();
                } else {
                    log_debug("idle...");
                }
            }
        }
    }
    Box::pin(move_with_interval(cfg, tx)).await;
}

#[inline(always)]
pub fn log_debug(msg: &str) {
    if cfg!(debug_assertions) {
        println!("{msg}");
    }
}
