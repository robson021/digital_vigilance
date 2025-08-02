use std::time::Duration;
use tray_item::{IconSource, TrayItem};

mod mouse_handler;

#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() {
    let minutes = 3;
    let label = &format!("Refresh time: {minutes} minutes");

    let mut tray = TrayItem::new("Digital Vigilance", IconSource::Resource("")).unwrap();
    tray.add_label(label).unwrap();

    tokio::spawn(async move {
        if cfg!(debug_assertions) {
            println!("Will move mouse every {minutes} minutes.");
        }
        let duration = Duration::from_secs(minutes * 60);
        loop {
            tokio::time::sleep(duration).await;
            mouse_handler::move_silently()
        }
    });

    let inner = tray.inner_mut();
    inner.add_quit_item("Quit");
    inner.display();
}
