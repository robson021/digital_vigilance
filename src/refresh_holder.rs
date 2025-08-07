use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

pub type SharedConfig = Arc<Mutex<ConfigHolder>>;

pub struct ConfigHolder {
    pub refresh_time: Duration,
}

impl ConfigHolder {
    pub fn new(refresh_time_seconds: u64) -> SharedConfig {
        let cfg = ConfigHolder {
            refresh_time: Duration::from_secs(refresh_time_seconds),
        };
        Arc::new(Mutex::new(cfg))
    }

    #[inline(always)]
    pub fn set_refresh_time(&mut self, new_refresh: Duration) {
        self.refresh_time = new_refresh;
    }
}
