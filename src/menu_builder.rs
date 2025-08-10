use crate::config_holder::{SharedConfig, TaskUptime};
use crate::popup_notification::show_message;
use crate::{APP_NAME, FromMin, log_debug};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, SystemTime};
use tokio::sync::broadcast::Sender;
use tray_item::{IconSource, TrayItem};

pub fn build_menu(config: SharedConfig, tx: Sender<()>) {
    let mut tray = TrayItem::new(APP_NAME, IconSource::Resource("")).unwrap();
    {
        tray.add_label("Keep awake for:").unwrap();

        for task_uptime in [
            TaskUptime::Timed(Duration::from_min(5)),
            TaskUptime::Timed(Duration::from_min(15)),
            TaskUptime::Timed(Duration::from_min(30)),
            TaskUptime::Timed(Duration::from_min(45)),
            TaskUptime::Timed(Duration::from_min(60)),
            TaskUptime::Infinite,
        ] {
            let config = config.clone();
            let tx = tx.clone();
            let refresh_action = move || set_new_refresh(&config, task_uptime, &tx);
            let label = &task_uptime.to_string();
            tray.add_menu_item(label, refresh_action).unwrap();
        }

        tray.add_label("Info:").unwrap();
        tray.add_menu_item("Time left", move || {
            let config = config.clone();
            thread::spawn(move || {
                let (time_left, start_time) = {
                    let guard = config.blocking_lock();
                    (guard.time_left(), guard.start_time)
                };
                match start_time {
                    Some(time) => {
                        let elapsed = SystemTime::now().duration_since(time).unwrap();
                        show_message(&format!(
                            "Elapsed: {} min.\nLeft: {} min",
                            elapsed.as_minutes(),
                            time_left.as_minutes(),
                        ));
                    }
                    None => show_message("No task is running."),
                }
            });
        })
        .unwrap();

        tray.add_menu_item("About the app", || {
            show_message("Source code:\ngithub.com/robson021/digital_vigilance");
        })
        .unwrap();

        tray.add_label("Stop:").unwrap();
    }

    let inner = tray.inner_mut();
    inner.add_quit_item("Quit");
    inner.display();
}

fn set_new_refresh(cfg: &SharedConfig, uptime: TaskUptime, tx: &Sender<()>) {
    log_debug(&format!("New refresh time: {:?}", uptime));
    let cfg = Arc::clone(cfg);
    let tx = tx.clone();
    thread::spawn(move || {
        cfg.blocking_lock().set_refresh_time(uptime);
        tx.send(()).unwrap();
    });
}
