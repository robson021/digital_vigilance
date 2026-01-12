use crate::config_holder::{SharedConfig, TaskUptime};
use crate::popup_notification::show_message;
use crate::{APP_NAME, ToTimeFormat, log_debug};
use std::time::{Duration, SystemTime};
use tokio::sync::broadcast::Sender;
use tray_item::{IconSource, TrayItem};

pub fn build_menu(config: SharedConfig, tx: Sender<()>) {
    let mut tray = TrayItem::new(APP_NAME, IconSource::Resource("")).unwrap();
    {
        tray.add_label("Keep awake for:").unwrap();

        let uptimes = [5, 8, 13, 21, 34, 55, 89] // Fibonacci
            .iter()
            .map(|x| TaskUptime::Timed(Duration::from_min(*x)))
            .collect::<Vec<_>>();

        for task_uptime in uptimes {
            let config = config.clone();
            let tx = tx.clone();
            let refresh_action = move || set_new_refresh(&config, task_uptime, &tx);
            let label = &task_uptime.to_string();
            tray.add_menu_item(label, refresh_action).unwrap();
        }

        tray.add_label("Info:").unwrap();
        tray.add_menu_item("Time left", move || {
            let config = config.clone();
            tokio::spawn(async move {
                let (time_left, start_time) = {
                    let task_metadata = config.lock().await;
                    (task_metadata.time_left(), task_metadata.start_time)
                };
                match start_time {
                    Some(time) => {
                        let elapsed = SystemTime::now().duration_since(time).unwrap();
                        show_message(&format!(
                            "Elapsed: {}.\nLeft: {}.",
                            elapsed.as_string(),
                            time_left.as_string(),
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
    let cfg = cfg.clone();
    let tx = tx.clone();

    tokio::spawn(async move {
        cfg.lock().await.set_refresh_time(uptime);
        tx.send(()).unwrap();
    });
}
