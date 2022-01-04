use std::ops::{Add, Sub};
use super::UTimestamp;

#[derive(Default, PartialEq, PartialOrd)]
pub struct UDuration {
    useconds_: i64,
}

impl UDuration {
    fn new(useconds: i64) -> Self {
        Self { useconds_: useconds }
    }

    pub fn as_microseconds(&self) -> i64 {
        self.useconds_
    }

    pub fn as_milliseconds(&self) -> i64 {
        self.useconds_ / 1000
    }

    pub fn as_seconds(&self) -> i64 {
        self.useconds_ / 1000000
    }

    pub fn as_minutes(&self) -> i64 {
        self.useconds_ / (1000000 * 60)
    }

    pub fn as_hours(&self) -> i64 {
        self.useconds_ / (1000000 * 3600)
    }
}

impl Add<UDuration> for UDuration {
    type Output = UDuration;

    fn add(self, other: UDuration) -> UDuration {
        Self::new(self.useconds_ + other.useconds_)
    }
}

impl Add<UTimestamp> for UDuration {
    type Output = UTimestamp;

    fn add(self, other: UTimestamp) -> UTimestamp {
        other + self
    }
}

impl Sub<UDuration> for UDuration {
    type Output = UDuration;

    fn sub(self, other: UDuration) -> UDuration {
        Self::new(self.useconds_ - other.useconds_)
    }
}

pub fn microseconds(value: i64) -> UDuration {
    UDuration::new(value)
}

pub fn milliseconds(value: i64) -> UDuration {
    UDuration::new(value * 1000)
}

pub fn seconds(value: i64) -> UDuration {
    UDuration::new(value * 1000000)
}

pub fn minutes(value: i64) -> UDuration {
    UDuration::new(value * 1000000 * 60)
}

pub fn hours(value: i64) -> UDuration {
    UDuration::new(value * 1000000 * 3600)
}
