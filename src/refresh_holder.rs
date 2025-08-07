use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

pub type SharedConfig = Arc<Mutex<ConfigHolder>>;

pub struct ConfigHolder {
    pub uptime: Duration,
}

impl ConfigHolder {
    pub fn new(uptime_seconds: u64) -> SharedConfig {
        let cfg = ConfigHolder {
            uptime: Duration::from_secs(uptime_seconds),
        };
        Arc::new(Mutex::new(cfg))
    }

    #[inline(always)]
    pub fn set_refresh_time(&mut self, new_refresh: Duration) {
        self.uptime = new_refresh;
    }
}
