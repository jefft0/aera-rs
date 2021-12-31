use crate::core::UTimestamp;
use crate::core::UDuration;
use crate::core::u_duration::microseconds;

pub struct Utils {
}

impl Utils {
  /**
   * Make a string from (timestamp - time_reference) in the form XXXs:YYYms:ZZZus, with a minus
   * sign if it is negative.
   * \param timestamp The time stamp.
   * \param time_reference The reference time to subtract from timestamp, usuall the session start
   * time.
   * We do this because timestamp is seconds since 01/01/1970, so the seconds would be very large.
   * \return The formatted time string.
   */
   pub fn to_string_s_ms_us(timestamp: UTimestamp, time_reference: UTimestamp) -> String {
      let duration = timestamp - time_reference;
      let t = duration.as_microseconds().abs() as u64;

      let us = t % 1000;
      let ms = t / 1000;
      let s = ms / 1000;
      let ms = ms % 1000;
  
      let sign = if duration < microseconds(0) { "-" } else { "" };
      format!("{}{}s:{}ms:{}us", sign, s, ms, us)
   }

  /**
   * Make a string from duration in the form XXXus, with a minus sign if it is negative. However if
   * the microseconds portion is zero, then use YYYms. Or if the microseconds and milliseconds
   * portions are zero, then use ZZZs. (This is the complement to how the compiler parses
   * durations.)
   * \param duration The duration.
   * \return The formatted time string.
   */
   pub fn to_string_us(duration: UDuration) -> String {
      let us = duration.as_microseconds().abs() as u64;

      let sign = if duration < microseconds(0) { "-" } else { "" };
      if us % 1000 != 0 {
          format!("{}{}us", sign, us)
      }
      else {
          let ms = us / 1000;
          if ms % 1000 != 0 {
            format!("{}{}ms", sign, ms)
          }
          else {
              let s = ms / 1000;
              format!("{}{}s", sign, s)
          }
      }
   }

   pub fn relative_time(t: UTimestamp) -> String {
        // TODO: Use module static time_reference
        let debug_time_reference = UTimestamp::from_duration(microseconds(0));
        Self::to_string_s_ms_us(t, debug_time_reference)
    }
}
