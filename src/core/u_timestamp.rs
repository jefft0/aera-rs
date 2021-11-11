use std::ops::{Add, Sub};
use super::UDuration;
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

impl Add<UDuration> for UTimestamp {
    type Output = UTimestamp;

    fn add(self, other: UDuration) -> UTimestamp {
        Self::new(self.useconds_ + other.as_microseconds())
    }
}

impl Sub<UDuration> for UTimestamp {
    type Output = UTimestamp;

    fn sub(self, other: UDuration) -> UTimestamp {
        Self::new(self.useconds_ - other.as_microseconds())
    }
}

impl Sub<UTimestamp> for UTimestamp {
    type Output = UDuration;

    fn sub(self, other: Self) -> UDuration {
        microseconds(self.useconds_ - other.useconds_)
    }
}

pub fn from_duration(duration: UDuration) -> UTimestamp {
    UTimestamp::new(duration.as_microseconds())
}
