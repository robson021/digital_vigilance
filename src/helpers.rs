use std::time::Duration;

#[inline(always)]
pub fn log_debug(msg: &str) {
    if cfg!(debug_assertions) {
        println!("{msg}");
    }
}

pub trait FromMin {
    fn from_min(minutes: u64) -> Duration;
    fn to_minutes(self) -> u64;
}

impl FromMin for Duration {
    #[inline(always)]
    fn from_min(minutes: u64) -> Duration {
        Duration::from_secs(minutes * 60)
    }
    #[inline(always)]
    fn to_minutes(self) -> u64 {
        self.as_secs() / 60
    }
}
