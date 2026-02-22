use crate::error::PeriodError;
use chrono::{Duration, Local};

use crate::error::validate_non_negative;
use crate::relative::types::Relative;

/// Returns a [`Relative`] moment `weeks` weeks in the past.
///
/// A value of `0` returns the current date-time. Use `.as_date()` to get a
/// [`NaiveDate`] if you do not need the time component.
///
/// # Errors
/// Returns [`PeriodError::NegativeValue`] if `weeks` is negative.
/// Returns [`PeriodError::Overflow`] if the resulting date-time is out of range.
/// Use [`weeks_from_now`] for future offsets.
#[inline]
pub fn weeks_ago(weeks: i64) -> Result<Relative, PeriodError> {
    validate_non_negative(weeks, "weeks", "weeks_from_now")?;
    let duration = Duration::try_weeks(weeks).ok_or(PeriodError::Overflow {
        unit: "weeks",
        value: weeks,
    })?;
    Local::now()
        .checked_sub_signed(duration)
        .map(Relative)
        .ok_or(PeriodError::Overflow {
            unit: "weeks",
            value: weeks,
        })
}

/// Returns a [`Relative`] moment `weeks` weeks in the future.
///
/// A value of `0` returns the current date-time. Use `.as_date()` to get a
/// [`NaiveDate`] if you do not need the time component.
///
/// # Errors
/// Returns [`PeriodError::NegativeValue`] if `weeks` is negative.
/// Returns [`PeriodError::Overflow`] if the resulting date-time is out of range.
/// Use [`weeks_ago`] for past offsets.
#[inline]
pub fn weeks_from_now(weeks: i64) -> Result<Relative, PeriodError> {
    validate_non_negative(weeks, "weeks", "weeks_ago")?;
    let duration = Duration::try_weeks(weeks).ok_or(PeriodError::Overflow {
        unit: "weeks",
        value: weeks,
    })?;
    Local::now()
        .checked_add_signed(duration)
        .map(Relative)
        .ok_or(PeriodError::Overflow {
            unit: "weeks",
            value: weeks,
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::PeriodError;
    use chrono::{Duration, Local};

    #[test]
    fn test_weeks_ago_returns_correct_date() {
        let date = weeks_ago(2).unwrap().as_date();
        let expected = Local::now().date_naive() - Duration::weeks(2);
        assert_eq!(date, expected);
    }

    #[test]
    fn test_weeks_ago_with_zero_returns_today() {
        assert_eq!(weeks_ago(0).unwrap().as_date(), Local::now().date_naive());
    }

    #[test]
    fn test_weeks_ago_returns_correct_datetime() {
        let lower = Local::now() - Duration::weeks(2);
        let result = weeks_ago(2).unwrap().as_datetime();
        let upper = Local::now() - Duration::weeks(2);
        assert!(result >= lower);
        assert!(result <= upper);
    }

    #[test]
    fn test_weeks_ago_negative_returns_error() {
        assert_eq!(
            weeks_ago(-2).unwrap_err().to_string(),
            "weeks must be positive. Did you mean weeks_from_now(2)?"
        );
    }

    #[test]
    fn test_weeks_ago_overflow_returns_error() {
        assert!(weeks_ago(30_000_000).is_err());
    }

    #[test]
    fn test_weeks_from_now_returns_correct_date() {
        let date = weeks_from_now(2).unwrap().as_date();
        let expected = Local::now().date_naive() + Duration::weeks(2);
        assert_eq!(date, expected);
    }

    #[test]
    fn test_weeks_from_now_with_zero_returns_today() {
        assert_eq!(
            weeks_from_now(0).unwrap().as_date(),
            Local::now().date_naive()
        );
    }

    #[test]
    fn test_weeks_from_now_returns_correct_datetime() {
        let lower = Local::now() + Duration::weeks(2);
        let result = weeks_from_now(2).unwrap().as_datetime();
        let upper = Local::now() + Duration::weeks(2);
        assert!(result >= lower);
        assert!(result <= upper);
    }

    #[test]
    fn test_weeks_from_now_negative_returns_error() {
        assert_eq!(
            weeks_from_now(-2).unwrap_err().to_string(),
            "weeks must be positive. Did you mean weeks_ago(2)?"
        );
    }

    #[test]
    fn test_weeks_from_now_overflow_returns_error() {
        assert!(weeks_from_now(30_000_000).is_err());
    }

    #[test]
    fn test_weeks_from_now_negative_one() {
        let err = weeks_from_now(-1).unwrap_err();
        assert!(matches!(err, PeriodError::NegativeValue { value: 1, .. }));
    }

    #[test]
    fn test_weeks_ago_plus_n_equals_today() {
        let n = 3i64;
        let date = weeks_ago(n).unwrap().as_date();
        assert_eq!(date + Duration::weeks(n), Local::now().date_naive());
    }

    #[test]
    fn test_weeks_from_now_large_valid_value() {
        assert!(weeks_from_now(52).is_ok());
    }
}
