use crate::core::UTimestamp;
use crate::core::u_duration::microseconds;

pub fn relative_time(t: UTimestamp) -> String {
    // TODO: Use module static time_reference
    let debug_time_reference = UTimestamp::from_duration(microseconds(0));
    crate::core::time::to_string_seconds(t - debug_time_reference)
}
