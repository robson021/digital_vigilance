use crate::APP_NAME;
use crate::config_holder::TaskUptime;
use notify_rust::Notification;

#[inline]
pub fn show_time_remaining_notification(uptime: &TaskUptime) {
    show_message(&format!("Will stay vigilant for {uptime}."));
}

pub fn show_message(message: &str) {
    Notification::new()
        .summary(APP_NAME)
        .body(message)
        .show()
        .unwrap();
}
