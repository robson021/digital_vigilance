use crate::popup_notification::show_time_remaining_notification;
use crate::refresh_holder::SharedConfig;
use crate::{APP_NAME, log_debug};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::sync::broadcast::Sender;
use tray_item::{IconSource, TrayItem};

pub fn build_menu(config: &SharedConfig, tx: Sender<()>) {
    let mut tray = TrayItem::new(APP_NAME, IconSource::Resource("")).unwrap();
    tray.add_label("Keep awake for:").unwrap();

    for minutes in [3, 5, 15, 60, 120, 180] {
        let config = config.clone();
        let tx = tx.clone();
        let refresh_action = move || set_new_refresh(&config, minutes, &tx);
        let label = format!("{minutes} minutes");
        tray.add_menu_item(&label, refresh_action).unwrap();
    }

    let inner = tray.inner_mut();
    inner.add_quit_item("Quit");
    inner.display();
}

fn set_new_refresh(cfg: &SharedConfig, new_refresh_min: u64, tx: &Sender<()>) {
    log_debug(&format!("New refresh: {new_refresh_min} min"));
    let cfg = Arc::clone(cfg);
    let tx = tx.clone();
    thread::spawn(move || {
        cfg.blocking_lock()
            .set_refresh_time(Duration::from_secs(new_refresh_min * 60));
        tx.send(()).unwrap();
        show_time_remaining_notification(new_refresh_min);
    });
}
