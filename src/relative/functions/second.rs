use crate::error::PeriodError;
use chrono::{Duration, Local};

use crate::error::validate_non_negative;
use crate::relative::types::Relative;

/// Returns a [`Relative`] moment `seconds` seconds in the past.
///
/// A value of `0` returns the current date-time.
///
/// # Errors
/// Returns [`PeriodError::NegativeValue`] if `seconds` is negative.
/// Returns [`PeriodError::Overflow`] if the resulting date-time is out of range.
/// Use [`seconds_from_now`] for future offsets.
#[inline]
pub fn seconds_ago(seconds: i64) -> Result<Relative, PeriodError> {
    validate_non_negative(seconds, "seconds", "seconds_from_now")?;
    let duration = Duration::try_seconds(seconds).ok_or(PeriodError::Overflow {
        unit: "seconds",
        value: seconds,
    })?;
    Local::now()
        .checked_sub_signed(duration)
        .map(Relative)
        .ok_or(PeriodError::Overflow {
            unit: "seconds",
            value: seconds,
        })
}

/// Returns a [`Relative`] moment `seconds` seconds in the future.
///
/// A value of `0` returns the current date-time.
///
/// # Errors
/// Returns [`PeriodError::NegativeValue`] if `seconds` is negative.
/// Returns [`PeriodError::Overflow`] if the resulting date-time is out of range.
/// Use [`seconds_ago`] for past offsets.
#[inline]
pub fn seconds_from_now(seconds: i64) -> Result<Relative, PeriodError> {
    validate_non_negative(seconds, "seconds", "seconds_ago")?;
    let duration = Duration::try_seconds(seconds).ok_or(PeriodError::Overflow {
        unit: "seconds",
        value: seconds,
    })?;
    Local::now()
        .checked_add_signed(duration)
        .map(Relative)
        .ok_or(PeriodError::Overflow {
            unit: "seconds",
            value: seconds,
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Local};

    #[test]
    fn test_seconds_ago_returns_correct_datetime() {
        let lower = Local::now() - Duration::seconds(3);
        let result = seconds_ago(3).unwrap().as_datetime();
        let upper = Local::now() - Duration::seconds(3);
        assert!(result >= lower);
        assert!(result <= upper);
    }

    #[test]
    fn test_seconds_ago_with_zero_returns_now() {
        let before = Local::now();
        let result = seconds_ago(0).unwrap().as_datetime();
        let after = Local::now();
        assert!(result >= before);
        assert!(result <= after);
    }

    #[test]
    fn test_seconds_ago_negative_returns_error() {
        assert_eq!(
            seconds_ago(-3).unwrap_err().to_string(),
            "seconds must be positive. Did you mean seconds_from_now(3)?"
        );
    }

    #[test]
    fn test_seconds_ago_overflow_returns_error() {
        assert!(seconds_ago(i64::MAX).is_err());
    }

    #[test]
    fn test_seconds_from_now_returns_correct_datetime() {
        let lower = Local::now() + Duration::seconds(3);
        let result = seconds_from_now(3).unwrap().as_datetime();
        let upper = Local::now() + Duration::seconds(3);
        assert!(result >= lower);
        assert!(result <= upper);
    }

    #[test]
    fn test_seconds_from_now_with_zero_returns_now() {
        let before = Local::now();
        let result = seconds_from_now(0).unwrap().as_datetime();
        let after = Local::now();
        assert!(result >= before);
        assert!(result <= after);
    }

    #[test]
    fn test_seconds_from_now_negative_returns_error() {
        assert_eq!(
            seconds_from_now(-3).unwrap_err().to_string(),
            "seconds must be positive. Did you mean seconds_ago(3)?"
        );
    }

    #[test]
    fn test_seconds_from_now_overflow_returns_error() {
        assert!(seconds_from_now(i64::MAX).is_err());
    }

    #[test]
    fn test_seconds_ago_negative_one() {
        let err = seconds_ago(-1).unwrap_err();
        assert!(matches!(err, PeriodError::NegativeValue { value: 1, .. }));
    }

    #[test]
    fn test_seconds_ago_is_in_the_past() {
        assert!(seconds_ago(10).unwrap().as_datetime() < Local::now());
    }

    #[test]
    fn test_seconds_from_now_is_in_the_future() {
        assert!(seconds_from_now(10).unwrap().as_datetime() > Local::now());
    }
}
