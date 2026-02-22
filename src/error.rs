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
