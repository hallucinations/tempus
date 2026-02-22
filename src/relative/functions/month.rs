use crate::error::PeriodError;
use chrono::{Local, Months};

use crate::error::validate_non_negative;
use crate::relative::types::Relative;

/// Returns a [`Relative`] moment `months` calendar months in the past.
///
/// A value of `0` returns the current date-time. Use `.as_date()` to get a
/// [`chrono::NaiveDate`] if you do not need the time component.
///
/// # Errors
/// Returns [`PeriodError::NegativeValue`] if `months` is negative.
/// Returns [`PeriodError::Overflow`] if `months` exceeds [`u32::MAX`] or the resulting date-time is out of range.
/// Use [`months_from_now`] for future offsets.
#[inline]
pub fn months_ago(months: i64) -> Result<Relative, PeriodError> {
    validate_non_negative(months, "months", "months_from_now")?;
    let months_u32 = u32::try_from(months).map_err(|_| PeriodError::Overflow {
        unit: "months",
        value: months,
    })?;
    Local::now()
        .checked_sub_months(Months::new(months_u32))
        .map(Relative)
        .ok_or(PeriodError::Overflow {
            unit: "months",
            value: months,
        })
}

/// Returns a [`Relative`] moment `months` calendar months in the future.
///
/// A value of `0` returns the current date-time. Use `.as_date()` to get a
/// [`chrono::NaiveDate`] if you do not need the time component.
///
/// # Errors
/// Returns [`PeriodError::NegativeValue`] if `months` is negative.
/// Returns [`PeriodError::Overflow`] if `months` exceeds [`u32::MAX`] or the resulting date-time is out of range.
/// Use [`months_ago`] for past offsets.
#[inline]
pub fn months_from_now(months: i64) -> Result<Relative, PeriodError> {
    validate_non_negative(months, "months", "months_ago")?;
    let months_u32 = u32::try_from(months).map_err(|_| PeriodError::Overflow {
        unit: "months",
        value: months,
    })?;
    Local::now()
        .checked_add_months(Months::new(months_u32))
        .map(Relative)
        .ok_or(PeriodError::Overflow {
            unit: "months",
            value: months,
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::PeriodError;
    use chrono::{Local, Months};

    #[test]
    fn test_months_ago_returns_correct_date() {
        let date = months_ago(2).unwrap().as_date();
        let expected = Local::now().date_naive() - Months::new(2);
        assert_eq!(date, expected);
    }

    #[test]
    fn test_months_ago_with_zero_returns_today() {
        assert_eq!(months_ago(0).unwrap().as_date(), Local::now().date_naive());
    }

    #[test]
    fn test_months_ago_returns_correct_datetime() {
        let lower = Local::now().checked_sub_months(Months::new(2)).unwrap();
        let result = months_ago(2).unwrap().as_datetime();
        let upper = Local::now().checked_sub_months(Months::new(2)).unwrap();
        assert!(result >= lower);
        assert!(result <= upper);
    }

    #[test]
    fn test_months_ago_negative_returns_error() {
        assert_eq!(
            months_ago(-2).unwrap_err().to_string(),
            "months must be positive. Did you mean months_from_now(2)?"
        );
    }

    #[test]
    fn test_months_ago_overflow_returns_error() {
        let result = months_ago(5_000_000_000);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "months value 5000000000 is too large"
        );
    }

    #[test]
    fn test_months_from_now_returns_correct_date() {
        let date = months_from_now(2).unwrap().as_date();
        let expected = Local::now().date_naive() + Months::new(2);
        assert_eq!(date, expected);
    }

    #[test]
    fn test_months_from_now_with_zero_returns_today() {
        assert_eq!(
            months_from_now(0).unwrap().as_date(),
            Local::now().date_naive()
        );
    }

    #[test]
    fn test_months_from_now_returns_correct_datetime() {
        let lower = Local::now().checked_add_months(Months::new(2)).unwrap();
        let result = months_from_now(2).unwrap().as_datetime();
        let upper = Local::now().checked_add_months(Months::new(2)).unwrap();
        assert!(result >= lower);
        assert!(result <= upper);
    }

    #[test]
    fn test_months_from_now_negative_returns_error() {
        assert_eq!(
            months_from_now(-2).unwrap_err().to_string(),
            "months must be positive. Did you mean months_ago(2)?"
        );
    }

    #[test]
    fn test_months_from_now_overflow_returns_error() {
        let result = months_from_now(5_000_000_000);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "months value 5000000000 is too large"
        );
    }

    #[test]
    fn test_negative_value_error_has_correct_suggestion_field() {
        let err = months_ago(-3).unwrap_err();
        assert!(
            matches!(
                err,
                PeriodError::NegativeValue {
                    suggestion: "months_from_now",
                    ..
                }
            ),
            "expected suggestion='months_from_now', got {err:?}"
        );
    }

    #[test]
    fn test_overflow_error_has_correct_unit_field() {
        let err = months_ago(5_000_000_000).unwrap_err();
        assert!(
            matches!(err, PeriodError::Overflow { unit: "months", .. }),
            "expected unit='months', got {err:?}"
        );
    }

    #[test]
    fn test_overflow_error_has_correct_value_field() {
        let err = months_ago(5_000_000_000).unwrap_err();
        assert!(
            matches!(
                err,
                PeriodError::Overflow {
                    value: 5_000_000_000,
                    ..
                }
            ),
            "expected value=5_000_000_000, got {err:?}"
        );
    }

    #[test]
    fn test_period_error_negative_value_ne_overflow() {
        assert_ne!(
            crate::relative::functions::day::days_ago(-1).unwrap_err(),
            months_ago(5_000_000_000).unwrap_err()
        );
    }

    #[test]
    fn test_months_ago_is_in_the_past() {
        assert!(months_ago(1).unwrap().as_date() < Local::now().date_naive());
    }

    #[test]
    fn test_months_from_now_is_in_the_future() {
        assert!(months_from_now(1).unwrap().as_date() > Local::now().date_naive());
    }
}
