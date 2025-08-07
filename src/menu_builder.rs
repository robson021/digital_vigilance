use crate::APP_NAME;
use crate::popup_notification::show_notification;
use crate::refresh_holder::SharedConfig;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tray_item::{IconSource, TrayItem};

pub fn build_menu(config: &SharedConfig) {
    let mut tray = TrayItem::new(APP_NAME, IconSource::Resource("")).unwrap();
    tray.add_label("Refresh time:").unwrap();

    for minutes in [1, 3, 5] {
        let config = Arc::clone(config);
        let refresh_action = move || set_new_refresh(&config, minutes);
        let label = format!("{minutes} minutes");
        tray.add_menu_item(&label, refresh_action).unwrap();
    }

    let inner = tray.inner_mut();
    inner.add_quit_item("Quit");
    inner.display();
}

fn set_new_refresh(cfg: &SharedConfig, new_refresh_min: u64) {
    let cfg = Arc::clone(cfg);
    thread::spawn(move || {
        cfg.blocking_lock()
            .set_refresh_time(Duration::from_secs(new_refresh_min * 60));
        show_notification(new_refresh_min);
    });
}
