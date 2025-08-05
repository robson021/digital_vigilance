use crate::menu_builder::build_menu;
use crate::popup_notification::show_notification;
use crate::refresh_holder::ConfigHolder;
use std::sync::Arc;
use tokio::sync::Mutex;

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
        let duration = Arc::clone(&config);
        move_with_interval(duration).await;
    });
    show_notification(DEFAULT_REFRESH_TIME_SEC / 60);

    build_menu(&cloned_config);
}

async fn move_with_interval(duration: Arc<Mutex<ConfigHolder>>) {
    // todo: send some signal to break current loop and re-read duration
    loop {
        let duration = duration.lock().await.refresh_time;
        if cfg!(debug_assertions) {
            println!("Current duration: {}s", duration.as_secs());
        }
        tokio::time::sleep(duration).await;
        mouse_handler::move_silently();
    }
}
