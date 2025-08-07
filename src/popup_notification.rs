use crate::APP_NAME;
use notify_rust::Notification;

#[inline]
pub fn show_time_remaining_notification(time_in_minutes: u64) {
    show_message(&format!(
        "Will stay vigilant for {time_in_minutes} minutes."
    ));
}

pub fn show_message(message: &str) {
    Notification::new()
        .summary(APP_NAME)
        .body(message)
        .show()
        .unwrap();
}
