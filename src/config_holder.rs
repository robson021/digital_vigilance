use crate::config_holder::TaskUptime::Timed;
use crate::helpers::FromMin;
use std::fmt::Display;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::Mutex;

pub type SharedConfig = Arc<Mutex<VigilanceTaskMetadata>>;

#[derive(Debug, Clone, Copy)]
pub enum TaskUptime {
    Infinite,
    Timed(Duration),
}

impl Display for TaskUptime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let duration = match self {
            Timed(duration) => {
                let sec = duration.as_secs();
                if sec < 60 {
                    format!("{sec} seconds")
                } else {
                    format!("{} minutes", duration.as_minutes())
                }
            }
            TaskUptime::Infinite => "infinity".to_owned(),
        };
        write!(f, "{}", duration)
    }
}

pub struct VigilanceTaskMetadata {
    pub uptime: TaskUptime,
    pub start_time: Option<SystemTime>,
}

impl VigilanceTaskMetadata {
    // pub fn new_infinite() -> SharedConfig { Arc::new(Mutex::new(ConfigHolder { uptime: TaskUptime::Infinite, })) }

    pub fn new_timed(uptime_seconds: u64) -> SharedConfig {
        Arc::new(Mutex::new(VigilanceTaskMetadata {
            uptime: Timed(Duration::from_secs(uptime_seconds)),
            start_time: None,
        }))
    }

    pub fn time_left(&self) -> Duration {
        match self.uptime {
            TaskUptime::Infinite => Duration::MAX,
            Timed(duration) => {
                let start_time = self.start_time.unwrap();
                let elapsed = SystemTime::now().duration_since(start_time).unwrap();
                duration.abs_diff(elapsed)
            }
        }
    }

    #[inline]
    pub fn set_start_time_to_now(&mut self) {
        self.start_time = Some(SystemTime::now());
    }

    #[inline]
    pub fn set_refresh_time(&mut self, new_uptime: TaskUptime) {
        self.uptime = new_uptime;
    }
}
