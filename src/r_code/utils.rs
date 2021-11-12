use crate::core::UTimestamp;
use crate::core::u_duration::microseconds;
use crate::core::u_timestamp::from_duration;

pub fn relative_time(t: UTimestamp) -> String {
    // TODO: Use module static time_reference.
    let debug_time_reference = from_duration(microseconds(0));
    crate::core::time::to_string_seconds(t - debug_time_reference)
}
