use crate::error::PeriodError;
use chrono::{Duration, Local};

use super::validate_non_negative;
use crate::relative::types::Relative;

/// Returns a [`Relative`] moment `minutes` minutes in the past.
///
/// A value of `0` returns the current date-time.
///
/// # Errors
/// Returns [`PeriodError::NegativeValue`] if `minutes` is negative.
/// Returns [`PeriodError::Overflow`] if the resulting date-time is out of range.
/// Use [`minutes_from_now`] for future offsets.
#[inline]
pub fn minutes_ago(minutes: i64) -> Result<Relative, PeriodError> {
    validate_non_negative(minutes, "minutes", "minutes_from_now")?;
    let duration = Duration::try_minutes(minutes).ok_or(PeriodError::Overflow {
        unit: "minutes",
        value: minutes,
    })?;
    Local::now()
        .checked_sub_signed(duration)
        .map(Relative)
        .ok_or(PeriodError::Overflow {
            unit: "minutes",
            value: minutes,
        })
}

/// Returns a [`Relative`] moment `minutes` minutes in the future.
///
/// A value of `0` returns the current date-time.
///
/// # Errors
/// Returns [`PeriodError::NegativeValue`] if `minutes` is negative.
/// Returns [`PeriodError::Overflow`] if the resulting date-time is out of range.
/// Use [`minutes_ago`] for past offsets.
#[inline]
pub fn minutes_from_now(minutes: i64) -> Result<Relative, PeriodError> {
    validate_non_negative(minutes, "minutes", "minutes_ago")?;
    let duration = Duration::try_minutes(minutes).ok_or(PeriodError::Overflow {
        unit: "minutes",
        value: minutes,
    })?;
    Local::now()
        .checked_add_signed(duration)
        .map(Relative)
        .ok_or(PeriodError::Overflow {
            unit: "minutes",
            value: minutes,
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::PeriodError;
    use chrono::{Duration, Local};

    #[test]
    fn test_minutes_ago_returns_correct_datetime() {
        let lower = Local::now() - Duration::minutes(3);
        let result = minutes_ago(3).unwrap().as_datetime();
        let upper = Local::now() - Duration::minutes(3);
        assert!(result >= lower);
        assert!(result <= upper);
    }

    #[test]
    fn test_minutes_ago_with_zero_returns_now() {
        let before = Local::now();
        let result = minutes_ago(0).unwrap().as_datetime();
        let after = Local::now();
        assert!(result >= before);
        assert!(result <= after);
    }

    #[test]
    fn test_minutes_ago_negative_returns_error() {
        assert_eq!(
            minutes_ago(-3).unwrap_err().to_string(),
            "minutes must be positive. Did you mean minutes_from_now(3)?"
        );
    }

    #[test]
    fn test_minutes_ago_overflow_returns_error() {
        assert!(minutes_ago(i64::MAX).is_err());
    }

    #[test]
    fn test_minutes_from_now_returns_correct_datetime() {
        let lower = Local::now() + Duration::minutes(3);
        let result = minutes_from_now(3).unwrap().as_datetime();
        let upper = Local::now() + Duration::minutes(3);
        assert!(result >= lower);
        assert!(result <= upper);
    }

    #[test]
    fn test_minutes_from_now_with_zero_returns_now() {
        let before = Local::now();
        let result = minutes_from_now(0).unwrap().as_datetime();
        let after = Local::now();
        assert!(result >= before);
        assert!(result <= after);
    }

    #[test]
    fn test_minutes_from_now_negative_returns_error() {
        assert_eq!(
            minutes_from_now(-3).unwrap_err().to_string(),
            "minutes must be positive. Did you mean minutes_ago(3)?"
        );
    }

    #[test]
    fn test_minutes_from_now_overflow_returns_error() {
        assert!(minutes_from_now(i64::MAX).is_err());
    }

    #[test]
    fn test_minutes_from_now_negative_large() {
        let err = minutes_from_now(-1_000).unwrap_err();
        assert!(matches!(
            err,
            PeriodError::NegativeValue { value: 1_000, .. }
        ));
    }
}
