use crate::APP_NAME;
use notify_rust::Notification;

pub fn show_notification(time_in_minutes: u64) {
    let message = &format!("Will stay vigilant for {time_in_minutes} minutes.");
    Notification::new()
        .summary(APP_NAME)
        .body(message)
        .show()
        .unwrap();
}
