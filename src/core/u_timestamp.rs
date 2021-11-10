use super::u_duration::UDuration;
use super::u_duration::microseconds;

pub struct UTimestamp {
    useconds_: i64,
}

impl UTimestamp {
    fn new(useconds: i64) -> Self {
        Self { useconds_: useconds }
    }

    pub fn time_since_epoch(&self) -> UDuration {
        microseconds(self.useconds_)
    }
}

pub fn from_duration(duration: UDuration) -> UTimestamp {
    UTimestamp::new(duration.as_microseconds())
}
