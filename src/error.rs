use std::fmt;

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum PeriodError {
    NegativeValue {
        unit: &'static str,
        suggestion: &'static str,
        value: u64,
    },
    Overflow {
        unit: &'static str,
        value: i64,
    },
}

impl std::error::Error for PeriodError {}

impl fmt::Display for PeriodError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PeriodError::NegativeValue {
                unit,
                suggestion,
                value,
            } => {
                write!(
                    f,
                    "{unit} must be positive. Did you mean {suggestion}({value})?"
                )
            }
            PeriodError::Overflow { unit, value } => {
                write!(f, "{unit} value {value} is too large")
            }
        }
    }
}
/// Validates that `value` is non-negative, returning a [`PeriodError::NegativeValue`] if not.
///
/// # Errors
///
/// Returns [`PeriodError::NegativeValue`] when `value < 0`.
pub(crate) fn validate_non_negative(
    value: i64,
    unit: &'static str,
    suggestion: &'static str,
) -> Result<(), PeriodError> {
    if value < 0 {
        return Err(PeriodError::NegativeValue {
            unit,
            suggestion,
            value: value.unsigned_abs(),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_non_negative_zero_is_ok() {
        assert!(validate_non_negative(0, "x", "y").is_ok());
    }

    #[test]
    fn test_validate_non_negative_positive_is_ok() {
        assert!(validate_non_negative(100, "x", "y").is_ok());
    }

    #[test]
    fn test_validate_non_negative_negative_is_err() {
        let err = validate_non_negative(-7, "days", "days_from_now").unwrap_err();
        assert!(matches!(
            err,
            PeriodError::NegativeValue {
                unit: "days",
                suggestion: "days_from_now",
                value: 7,
            }
        ));
    }
}
