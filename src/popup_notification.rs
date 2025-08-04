use crate::APP_NAME;
use notify_rust::Notification;

pub fn show_notification(refresh_time_min: u64) {
    let message = &format!("Refreshed time: every {refresh_time_min} minutes.");
    Notification::new()
        .summary(APP_NAME)
        .body(message)
        .show()
        .unwrap();
}
