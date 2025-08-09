use crate::config_holder::TaskUptime::Timed;
use crate::helpers::FromMin;
use std::fmt::Display;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

pub type SharedConfig = Arc<Mutex<ConfigHolder>>;

#[derive(Debug, Clone, Copy)]
pub enum TaskUptime {
    Infinite,
    Timed(Duration),
}

impl Display for TaskUptime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let duration = match self {
            Timed(d) => {
                let sec = d.as_secs();
                if sec < 60 {
                    format!("{sec} seconds")
                } else {
                    format!("{} minutes", d.as_minutes())
                }
            }
            TaskUptime::Infinite => "infinity".to_owned(),
        };
        write!(f, "{}", duration)
    }
}

pub struct ConfigHolder {
    pub uptime: TaskUptime,
}

impl ConfigHolder {
    // pub fn new_infinite() -> SharedConfig { Arc::new(Mutex::new(ConfigHolder { uptime: TaskUptime::Infinite, })) }

    pub fn new_timed(uptime_seconds: u64) -> SharedConfig {
        Arc::new(Mutex::new(ConfigHolder {
            uptime: Timed(Duration::from_secs(uptime_seconds)),
        }))
    }

    #[inline(always)]
    pub fn set_refresh_time(&mut self, new_uptime: TaskUptime) {
        self.uptime = new_uptime;
    }
}
