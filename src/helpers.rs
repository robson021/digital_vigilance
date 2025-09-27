use humanize_duration::Truncate;
use humanize_duration::prelude::DurationExt;
use std::time::Duration;

#[inline(always)]
pub fn log_debug(msg: &str) {
    if cfg!(debug_assertions) {
        println!("{msg}");
    }
}

pub trait ToTimeFormat {
    fn from_min(minutes: u64) -> Duration;
    fn as_minutes_rounded(&self) -> u64;
    fn as_string(&self) -> String;
}

impl ToTimeFormat for Duration {
    #[inline]
    fn from_min(minutes: u64) -> Duration {
        Duration::from_secs(minutes * 60)
    }

    #[inline]
    fn as_minutes_rounded(&self) -> u64 {
        self.as_secs() / 60
    }

    #[inline]
    fn as_string(&self) -> String {
        self.human(Truncate::Second).to_string()
    }
}
