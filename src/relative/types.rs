use chrono::{DateTime, Local, NaiveDate, NaiveTime};

/// A resolved point in time returned by every relative-date function.
///
/// Call `.as_datetime()`, `.as_date()`, or `.as_time()` to extract the
/// representation you need, or rely on the `From` / `Into` conversions.
///
/// # Example
///
/// ```rust
/// # fn main() -> Result<(), period::PeriodError> {
/// let r = period::days_ago(3)?;
/// let date     = r.as_date();     // NaiveDate       - just the calendar day
/// let datetime = r.as_datetime(); // DateTime<Local> - full timestamp
/// let time     = r.as_time();     // NaiveTime       - just the clock reading
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Relative(pub(super) DateTime<Local>);

impl Relative {
    /// The full local date-time.
    #[inline]
    #[must_use]
    pub fn as_datetime(self) -> DateTime<Local> {
        self.0
    }

    /// The calendar date, discarding the time-of-day component.
    #[inline]
    #[must_use]
    pub fn as_date(self) -> NaiveDate {
        self.0.date_naive()
    }

    /// The time-of-day, discarding the date component.
    #[inline]
    #[must_use]
    pub fn as_time(self) -> NaiveTime {
        self.0.time()
    }
}

impl From<Relative> for DateTime<Local> {
    fn from(r: Relative) -> Self {
        r.0
    }
}

impl From<Relative> for NaiveDate {
    fn from(r: Relative) -> Self {
        r.0.date_naive()
    }
}

impl From<Relative> for NaiveTime {
    fn from(r: Relative) -> Self {
        r.0.time()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::relative::functions::{days_ago, hours_ago, minutes_ago, seconds_ago};
    use chrono::Duration;

    #[test]
    fn test_relative_as_date_matches_naive_date() {
        let r = days_ago(1).unwrap();
        let expected = Local::now().date_naive() - Duration::days(1);
        assert_eq!(r.as_date(), expected);
        assert_eq!(NaiveDate::from(r), expected);
    }

    #[test]
    fn test_relative_as_datetime_matches_datetime() {
        let lower = Local::now() - Duration::hours(1);
        let r = hours_ago(1).unwrap();
        let upper = Local::now() - Duration::hours(1);
        let dt: DateTime<Local> = r.into();
        assert!(dt >= lower);
        assert!(dt <= upper);
    }

    #[test]
    fn test_relative_as_time_is_consistent_with_as_datetime() {
        let r = hours_ago(2).unwrap();
        assert_eq!(r.as_time(), r.as_datetime().time());
    }

    #[test]
    fn test_relative_past_is_less_than_future() {
        let past = days_ago(1).unwrap();
        let future = crate::relative::functions::days_from_now(1).unwrap();
        assert!(past < future);
    }

    #[test]
    fn test_relative_is_copy_can_use_twice() {
        let r = hours_ago(1).unwrap();
        let dt1 = r.as_datetime(); // uses Copy, r not moved
        let dt2 = r.as_datetime();
        assert_eq!(dt1, dt2);
    }

    #[test]
    fn test_relative_clone_equals_original() {
        let r = minutes_ago(5).unwrap();
        #[allow(clippy::clone_on_copy)]
        let cloned = r.clone();
        assert_eq!(r, cloned);
    }

    #[test]
    fn test_relative_debug_is_non_empty() {
        let r = seconds_ago(10).unwrap();
        assert!(!format!("{r:?}").is_empty());
    }

    #[test]
    fn test_relative_into_naive_time() {
        let r = hours_ago(1).unwrap();
        let t: NaiveTime = r.into();
        assert_eq!(t, r.as_time());
    }

    #[test]
    fn test_relative_into_naive_date() {
        let r = days_ago(3).unwrap();
        let d: NaiveDate = r.into();
        assert_eq!(d, r.as_date());
    }

    #[test]
    fn test_relative_as_time_matches_as_datetime_time() {
        let r = minutes_ago(90).unwrap();
        assert_eq!(r.as_time(), r.as_datetime().time());
    }

    #[test]
    fn test_relative_ordering_same_instant_is_equal() {
        let r = seconds_ago(0).unwrap();
        assert_eq!(r, r);
    }
}
