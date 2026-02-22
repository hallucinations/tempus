use chrono::NaiveDate;

/// Converts a NaiveDate to a string in "YYYY-MM-DD" format.
#[must_use]
#[inline]
pub fn to_date_string(date: NaiveDate) -> String {
    date.format("%Y-%m-%d").to_string()
}

/// Converts a NaiveDate to a string in "Month Day, Year" format (e.g., "February 22, 2026").
/// This uses the full month name.
/// Note: This will be in English regardless of locale.
/// For locale-aware formatting, consider using the `chrono_locale` crate.
/// Example: `to_long_date(NaiveDate::from_ymd_opt(2026, 2, 22).unwrap())` returns "February 22, 2026".
/// This is a simple wrapper around `chrono`'s formatting capabilities.
/// The format string "%B %e, %Y" means:
/// - %B: Full month name (e.g., "February")
/// - %e: Day of the month, space-padded (e.g., "22")
/// - %Y: Year with century (e.g., "2026")
#[must_use]
#[inline]
pub fn to_long_date(date: NaiveDate) -> String {
    date.format("%B %e, %Y").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_to_date_string() {
        let date = NaiveDate::from_ymd_opt(2026, 2, 22).unwrap();
        assert_eq!(to_date_string(date), "2026-02-22");
    }

    #[test]
    fn test_to_long_date() {
        let date = NaiveDate::from_ymd_opt(2026, 2, 22).unwrap();
        assert_eq!(to_long_date(date), "February 22, 2026");
    }

    #[test]
    fn test_to_long_date_with_single_digit_day() {
        let date = NaiveDate::from_ymd_opt(2026, 2, 5).unwrap();
        assert_eq!(to_long_date(date), "February  5, 2026");
    }
}
