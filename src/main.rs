use crate::menu_builder::build_menu;
use crate::popup_notification::show_notification;
use crate::refresh_holder::RefreshHolder;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

mod menu_builder;
mod mouse_handler;
mod popup_notification;
mod refresh_holder;

const APP_NAME: &str = "Digital Vigilance";
const DEFAULT_REFRESH_TIME_SEC: u64 = 3 * 60;

#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() {
    let initial_refresh = Arc::new(Mutex::new(RefreshHolder {
        duration: Duration::from_secs(DEFAULT_REFRESH_TIME_SEC),
    }));
    let cloned_refresh = Arc::clone(&initial_refresh);

    tokio::spawn(async move {
        let duration = Arc::clone(&initial_refresh);
        move_with_interval(duration).await;
    });
    show_notification(DEFAULT_REFRESH_TIME_SEC / 60);
    build_menu(&cloned_refresh);
}

async fn move_with_interval(duration: Arc<Mutex<RefreshHolder>>) {
    // todo: send some signal to break current loop and re-read duration
    loop {
        let duration = duration.lock().await.duration;
        if cfg!(debug_assertions) {
            println!("Current duration: {}s", duration.as_secs());
        }
        tokio::time::sleep(duration).await;
        mouse_handler::move_silently();
    }
}
