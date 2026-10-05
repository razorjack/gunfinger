//! Clock times such as `2:55` or `1:02:03`.

use std::time::Duration;

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error("`{0}` is not a time; use seconds (`95`, `95.5`), M:SS (`1:35`) or H:MM:SS (`1:01:35`)")]
pub struct TimecodeError(pub String);

/// Parses plain seconds, `M:SS` or `H:MM:SS`. Seconds may carry a fraction.
pub fn parse_timecode(text: &str) -> Result<Duration, TimecodeError> {
    let invalid = || TimecodeError(text.to_owned());
    let mut fields: Vec<&str> = text.trim().split(':').collect();
    let seconds = fields.pop().and_then(decimal).ok_or_else(invalid)?;
    let minutes = match fields.as_slice() {
        [] => 0,
        [minutes] => whole(minutes).ok_or_else(invalid)?,
        [hours, minutes] => {
            let minutes = whole(minutes)
                .filter(|&minutes| minutes < 60)
                .ok_or_else(invalid)?;
            whole(hours).ok_or_else(invalid)? * 60 + minutes
        }
        _ => return Err(invalid()),
    };
    if !fields.is_empty() && seconds >= 60.0 {
        return Err(invalid());
    }
    Ok(Duration::from_secs(minutes * 60) + Duration::from_secs_f64(seconds))
}

/// Digits only: no sign, no fraction.
fn whole(text: &str) -> Option<u64> {
    is_digits(text).then(|| text.parse().ok()).flatten()
}

/// Digits with an optional fraction: no sign, exponent or `inf`.
fn decimal(text: &str) -> Option<f64> {
    let (integer, fraction) = text.split_once('.').unwrap_or((text, "0"));
    (is_digits(integer) && is_digits(fraction))
        .then(|| text.parse().ok())
        .flatten()
}

fn is_digits(text: &str) -> bool {
    !text.is_empty() && text.bytes().all(|byte| byte.is_ascii_digit())
}

/// Formats as `M:SS` or `H:MM:SS`, rounding down to the second.
pub fn format_timecode(time: Duration) -> String {
    let total = time.as_secs();
    let (hours, minutes, seconds) = (total / 3600, total / 60 % 60, total % 60);
    if hours > 0 {
        format!("{hours}:{minutes:02}:{seconds:02}")
    } else {
        format!("{minutes}:{seconds:02}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seconds(text: &str) -> f64 {
        parse_timecode(text).unwrap().as_secs_f64()
    }

    #[test]
    fn clock_forms_are_accepted() {
        assert_eq!(seconds("0:00"), 0.0);
        assert_eq!(seconds("2:55"), 175.0);
        assert_eq!(seconds("49:54"), 2994.0);
        assert_eq!(seconds("1:02:03"), 3723.0);
        assert_eq!(seconds("95"), 95.0);
        assert_eq!(seconds("95.5"), 95.5);
        assert_eq!(seconds("1:05.5"), 65.5);
    }

    #[test]
    fn malformed_times_are_rejected() {
        for text in [
            "", "1:60", "1:75:00", "a:00", "1:-5", "-3", "1::2", "1:2:3:4", "1:+5", "inf",
        ] {
            assert!(parse_timecode(text).is_err(), "{text} was accepted");
        }
    }

    #[test]
    fn formatting_uses_hours_only_when_needed() {
        assert_eq!(format_timecode(Duration::from_secs(175)), "2:55");
        assert_eq!(format_timecode(Duration::from_secs(3723)), "1:02:03");
        assert_eq!(format_timecode(Duration::from_millis(59_999)), "0:59");
    }
}
