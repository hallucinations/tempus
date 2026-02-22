pub mod day;
pub mod hour;
pub mod minute;
pub mod month;
pub mod second;
pub mod week;
pub mod year;

pub use day::{days_ago, days_from_now, tomorrow, yesterday};
pub use hour::{hours_ago, hours_from_now};
pub use minute::{minutes_ago, minutes_from_now};
pub use month::{months_ago, months_from_now};
pub use second::{seconds_ago, seconds_from_now};
pub use week::{weeks_ago, weeks_from_now};
pub use year::{years_ago, years_from_now};

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Local};

    // -- cross-unit equivalence -----------------------------------------------

    #[test]
    fn test_60_seconds_ago_same_date_as_1_minute_ago() {
        assert_eq!(
            seconds_ago(60).unwrap().as_date(),
            minutes_ago(1).unwrap().as_date()
        );
    }

    #[test]
    fn test_60_minutes_ago_same_date_as_1_hour_ago() {
        assert_eq!(
            minutes_ago(60).unwrap().as_date(),
            hours_ago(1).unwrap().as_date()
        );
    }

    #[test]
    fn test_24_hours_ago_same_date_as_1_day_ago() {
        assert_eq!(
            hours_ago(24).unwrap().as_date(),
            days_ago(1).unwrap().as_date()
        );
    }

    #[test]
    fn test_7_days_ago_same_date_as_1_week_ago() {
        assert_eq!(
            days_ago(7).unwrap().as_date(),
            weeks_ago(1).unwrap().as_date()
        );
    }

    #[test]
    fn test_12_months_ago_same_date_as_1_year_ago() {
        assert_eq!(
            months_ago(12).unwrap().as_date(),
            years_ago(1).unwrap().as_date()
        );
    }

    #[test]
    fn test_60_seconds_from_now_same_date_as_1_minute_from_now() {
        assert_eq!(
            seconds_from_now(60).unwrap().as_date(),
            minutes_from_now(1).unwrap().as_date()
        );
    }

    #[test]
    fn test_7_days_from_now_same_date_as_1_week_from_now() {
        assert_eq!(
            days_from_now(7).unwrap().as_date(),
            weeks_from_now(1).unwrap().as_date()
        );
    }

    #[test]
    fn test_12_months_from_now_same_date_as_1_year_from_now() {
        assert_eq!(
            months_from_now(12).unwrap().as_date(),
            years_from_now(1).unwrap().as_date()
        );
    }

    // -- large valid (non-overflow) values ------------------------------------

    #[test]
    fn test_seconds_ago_large_valid_value() {
        // 86 400 s = 1 day
        assert_eq!(
            seconds_ago(86_400).unwrap().as_date(),
            days_ago(1).unwrap().as_date()
        );
    }

    #[test]
    fn test_minutes_ago_large_valid_value() {
        // 1 440 min = 1 day
        assert_eq!(
            minutes_ago(1_440).unwrap().as_date(),
            days_ago(1).unwrap().as_date()
        );
    }

    // -- arithmetic round-trips (cross-cutting) --------------------------------

    #[test]
    fn test_days_ago_plus_n_equals_today() {
        let n = 5i64;
        let date = days_ago(n).unwrap().as_date();
        assert_eq!(date + Duration::days(n), Local::now().date_naive());
    }

    #[test]
    fn test_days_from_now_minus_n_equals_today() {
        let n = 5i64;
        let date = days_from_now(n).unwrap().as_date();
        assert_eq!(date - Duration::days(n), Local::now().date_naive());
    }
}
