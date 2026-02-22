use crate::error::PeriodError;
use chrono::{Duration, Local};

use super::validate_non_negative;
use crate::relative::types::Relative;

/// Returns a [`Relative`] moment `hours` hours in the past.
///
/// A value of `0` returns the current date-time.
///
/// # Errors
/// Returns [`PeriodError::NegativeValue`] if `hours` is negative.
/// Returns [`PeriodError::Overflow`] if the resulting date-time is out of range.
/// Use [`hours_from_now`] for future offsets.
#[inline]
pub fn hours_ago(hours: i64) -> Result<Relative, PeriodError> {
    validate_non_negative(hours, "hours", "hours_from_now")?;
    let duration = Duration::try_hours(hours).ok_or(PeriodError::Overflow {
        unit: "hours",
        value: hours,
    })?;
    Local::now()
        .checked_sub_signed(duration)
        .map(Relative)
        .ok_or(PeriodError::Overflow {
            unit: "hours",
            value: hours,
        })
}

/// Returns a [`Relative`] moment `hours` hours in the future.
///
/// A value of `0` returns the current date-time.
///
/// # Errors
/// Returns [`PeriodError::NegativeValue`] if `hours` is negative.
/// Returns [`PeriodError::Overflow`] if the resulting date-time is out of range.
/// Use [`hours_ago`] for past offsets.
#[inline]
pub fn hours_from_now(hours: i64) -> Result<Relative, PeriodError> {
    validate_non_negative(hours, "hours", "hours_ago")?;
    let duration = Duration::try_hours(hours).ok_or(PeriodError::Overflow {
        unit: "hours",
        value: hours,
    })?;
    Local::now()
        .checked_add_signed(duration)
        .map(Relative)
        .ok_or(PeriodError::Overflow {
            unit: "hours",
            value: hours,
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::PeriodError;
    use chrono::{Duration, Local};

    #[test]
    fn test_hours_ago_returns_correct_datetime() {
        let lower = Local::now() - Duration::hours(3);
        let result = hours_ago(3).unwrap().as_datetime();
        let upper = Local::now() - Duration::hours(3);
        assert!(result >= lower);
        assert!(result <= upper);
    }

    #[test]
    fn test_hours_ago_with_zero_returns_now() {
        let before = Local::now();
        let result = hours_ago(0).unwrap().as_datetime();
        let after = Local::now();
        assert!(result >= before);
        assert!(result <= after);
    }

    #[test]
    fn test_hours_ago_negative_returns_error() {
        assert_eq!(
            hours_ago(-3).unwrap_err().to_string(),
            "hours must be positive. Did you mean hours_from_now(3)?"
        );
    }

    #[test]
    fn test_hours_ago_overflow_returns_error() {
        assert!(hours_ago(i64::MAX).is_err());
    }

    #[test]
    fn test_hours_from_now_returns_correct_datetime() {
        let lower = Local::now() + Duration::hours(3);
        let result = hours_from_now(3).unwrap().as_datetime();
        let upper = Local::now() + Duration::hours(3);
        assert!(result >= lower);
        assert!(result <= upper);
    }

    #[test]
    fn test_hours_from_now_with_zero_returns_now() {
        let before = Local::now();
        let result = hours_from_now(0).unwrap().as_datetime();
        let after = Local::now();
        assert!(result >= before);
        assert!(result <= after);
    }

    #[test]
    fn test_hours_from_now_negative_returns_error() {
        assert_eq!(
            hours_from_now(-3).unwrap_err().to_string(),
            "hours must be positive. Did you mean hours_ago(3)?"
        );
    }

    #[test]
    fn test_hours_from_now_overflow_returns_error() {
        assert!(hours_from_now(i64::MAX).is_err());
    }

    #[test]
    fn test_hours_ago_negative_one_has_hours_unit() {
        let err = hours_ago(-1).unwrap_err();
        assert!(matches!(
            err,
            PeriodError::NegativeValue { unit: "hours", .. }
        ));
    }

    #[test]
    fn test_hours_ago_is_in_the_past() {
        assert!(hours_ago(1).unwrap().as_datetime() < Local::now());
    }

    #[test]
    fn test_hours_from_now_is_in_the_future() {
        assert!(hours_from_now(1).unwrap().as_datetime() > Local::now());
    }

    #[test]
    fn test_hours_ago_large_valid_value() {
        // 240 h = 10 days
        assert_eq!(
            hours_ago(240).unwrap().as_date(),
            crate::relative::functions::day::days_ago(10)
                .unwrap()
                .as_date()
        );
    }
}
