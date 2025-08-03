use notify_rust::Notification;
use std::time::Duration;
use tray_item::{IconSource, TrayItem};

mod mouse_handler;

const TITLE: &str = "Digital Vigilance";
const REFRESH_TIME_MINUTES: u64 = 3;

#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() {
    let mut tray = TrayItem::new(TITLE, IconSource::Resource("")).unwrap();
    let label = &format!("Refresh time - every {REFRESH_TIME_MINUTES} minutes");
    tray.add_label(label).unwrap();

    tokio::spawn(async move {
        if cfg!(debug_assertions) {
            println!("Async task spawned")
        }
        let duration = Duration::from_secs(REFRESH_TIME_MINUTES * 60);
        loop {
            tokio::time::sleep(duration).await;
            mouse_handler::move_silently()
        }
    });

    show_notification(label);

    let inner = tray.inner_mut();
    inner.add_quit_item("Quit");
    inner.display();
}

fn show_notification(message: &str) {
    Notification::new()
        .summary(TITLE)
        .body(message)
        .show()
        .unwrap();
}
