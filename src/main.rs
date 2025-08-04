use notify_rust::Notification;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::sync::Mutex;
use tray_item::{IconSource, TrayItem};

mod mouse_handler;

struct RefreshHolder {
    duration: Duration,
}

impl RefreshHolder {
    fn set(&mut self, new_refresh: Duration) {
        self.duration = new_refresh;
    }
}

const TITLE: &str = "Digital Vigilance";
const DEFAULT_REFRESH_TIME_SEC: u64 = 3 * 60;

#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() {
    let mut tray = TrayItem::new(TITLE, IconSource::Resource("")).unwrap();
    tray.add_label("Refresh time:").unwrap();

    let initial_duration = Arc::new(Mutex::new(RefreshHolder {
        duration: Duration::from_secs(DEFAULT_REFRESH_TIME_SEC),
    }));
    let refresh_holder_1 = Arc::clone(&initial_duration);
    let refresh_holder_2 = Arc::clone(&initial_duration);
    let refresh_holder_3 = Arc::clone(&initial_duration);

    tokio::spawn(async move {
        let duration = Arc::clone(&initial_duration);
        move_with_interval(duration).await;
    });
    show_notification(DEFAULT_REFRESH_TIME_SEC / 60);

    let refresh_1_min = move || set_new_refresh(&refresh_holder_1, 1);
    let refresh_3_min = move || set_new_refresh(&refresh_holder_2, 3);
    let refresh_5_min = move || set_new_refresh(&refresh_holder_3, 5);

    tray.add_menu_item("1 minute", refresh_1_min).unwrap();
    tray.add_menu_item("3 minutes", refresh_3_min).unwrap();
    tray.add_menu_item("5 minutes", refresh_5_min).unwrap();

    let inner = tray.inner_mut();
    inner.add_quit_item("Quit");
    inner.display();
}

fn set_new_refresh(holder: &Arc<Mutex<RefreshHolder>>, new_refresh_min: u64) {
    let duration = Arc::clone(holder);
    thread::spawn(move || {
        let refresh_time = new_refresh_min * 60;
        duration
            .blocking_lock()
            .set(Duration::from_secs(refresh_time));
        show_notification(new_refresh_min);
    });
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

fn show_notification(refresh_time_min: u64) {
    let message = &format!("Refreshed time: every {refresh_time_min} minutes.");
    Notification::new()
        .summary(TITLE)
        .body(message)
        .show()
        .unwrap();
}
