use std::time::Duration;

pub struct RefreshHolder {
    pub duration: Duration,
}

impl RefreshHolder {
    #[inline(always)]
    pub fn set(&mut self, new_refresh: Duration) {
        self.duration = new_refresh;
    }
}
