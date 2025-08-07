use crate::menu_builder::build_menu;
use crate::popup_notification::show_notification;
use crate::refresh_holder::{ConfigHolder, SharedConfig};
use std::sync::Arc;

mod menu_builder;
mod mouse_handler;
mod popup_notification;
mod refresh_holder;

const APP_NAME: &str = "Digital Vigilance";
const DEFAULT_REFRESH_TIME_SEC: u64 = 3 * 60;

#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() {
    let config = ConfigHolder::new(DEFAULT_REFRESH_TIME_SEC);
    let cloned_config = Arc::clone(&config);

    tokio::spawn(async move {
        move_with_interval(cloned_config).await;
    });
    show_notification(DEFAULT_REFRESH_TIME_SEC / 60);

    build_menu(&config);
}

async fn move_with_interval(cfg: SharedConfig) {
    let duration = cfg.lock().await.refresh_time;
    loop {
        if cfg!(debug_assertions) {
            println!("Current refresh time: {}s", duration.as_secs());
        }
        tokio::time::sleep(duration).await;
        mouse_handler::move_silently();
    }
}
