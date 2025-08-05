use crate::APP_NAME;
use crate::popup_notification::show_notification;
use crate::refresh_holder::ConfigHolder;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::sync::Mutex;
use tray_item::{IconSource, TrayItem};

pub fn build_menu(refresh_holder: &Arc<Mutex<ConfigHolder>>) {
    let mut tray = TrayItem::new(APP_NAME, IconSource::Resource("")).unwrap();
    {
        let refresh_holder_1 = Arc::clone(refresh_holder);
        let refresh_holder_2 = Arc::clone(refresh_holder);
        let refresh_holder_3 = Arc::clone(refresh_holder);

        let refresh_1_min = move || set_new_refresh(&refresh_holder_1, 1);
        let refresh_3_min = move || set_new_refresh(&refresh_holder_2, 3);
        let refresh_5_min = move || set_new_refresh(&refresh_holder_3, 5);

        tray.add_label("Refresh time:").unwrap();
        tray.add_menu_item("1 minute", refresh_1_min).unwrap();
        tray.add_menu_item("3 minutes", refresh_3_min).unwrap();
        tray.add_menu_item("5 minutes", refresh_5_min).unwrap();
    }
    let inner = tray.inner_mut();
    inner.add_quit_item("Quit");
    inner.display();
}

fn set_new_refresh(cfg: &Arc<Mutex<ConfigHolder>>, new_refresh_min: u64) {
    let cfg = Arc::clone(cfg);
    let refresh_time = new_refresh_min * 60;
    thread::spawn(move || {
        cfg.blocking_lock()
            .set_refresh_time(Duration::from_secs(refresh_time));
        show_notification(new_refresh_min);
    });
}
