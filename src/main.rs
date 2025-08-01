use std::thread;
use std::time::Duration;
use tray_item::{IconSource, TrayItem};

mod mouse_handler;

fn main() {
    let sleep_time = 3 * 60;
    let label = &format!("Refresh time: {sleep_time}s");

    let mut tray = TrayItem::new("Digital Vigilance", IconSource::Resource("")).unwrap();
    tray.add_label(label).unwrap();

    let inner = tray.inner_mut();
    inner.add_quit_item("Quit");

    thread::spawn(move || {
        println!("Will move mouse every {sleep_time} seconds.");
        let duration = Duration::from_secs(sleep_time);
        loop {
            thread::sleep(duration);
            mouse_handler::move_silently()
        }
    });

    inner.display();
}
