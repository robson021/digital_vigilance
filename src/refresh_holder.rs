use std::time::Duration;

pub struct RefreshHolder {
    pub(crate) duration: Duration,
}

impl RefreshHolder {
    pub fn set(&mut self, new_refresh: Duration) {
        self.duration = new_refresh;
    }
}
