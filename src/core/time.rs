use super::u_duration::UDuration;

// seconds:milliseconds:microseconds.
pub fn to_string_seconds(duration: UDuration) -> String {
    let t = duration.as_microseconds().abs() as u64;

    let us = t % 1000;
    let ms = t / 1000;
    let s = ms / 1000;
    let ms = ms % 1000;
  
    let negative = duration.as_microseconds() < 0;
    return format!("{}{}s:{}ms:{}us", if negative {"-"} else {""}, s, ms, us);
}
