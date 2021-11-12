use super::u_duration::UDuration;

// seconds:milliseconds:microseconds.
pub fn to_string_seconds(duration: UDuration) -> String {
  // The compiler doesn't support negative times. Assume durations are positive.
  // Take the absolute value to be sure.
  let t = duration.as_microseconds().abs() as u64;

    let us = t % 1000;
    let ms = t / 1000;
    let s = ms / 1000;
    let ms = ms % 1000;
  
    return format!("{}s:{}ms:{}us", s, ms, us);
}
