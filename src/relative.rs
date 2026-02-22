use chrono::{DateTime, Duration, Local, Months, NaiveDate};

pub fn seconds_ago(seconds: i64) -> DateTime<Local> {
    Local::now() - Duration::seconds(seconds)
}

pub fn seconds_from_now(seconds: i64) -> DateTime<Local> {
    Local::now() + Duration::seconds(seconds)
}

pub fn minutes_ago(minutes: i64) -> DateTime<Local> {
    Local::now() - Duration::minutes(minutes)
}

pub fn minutes_from_now(minutes: i64) -> DateTime<Local> {
    Local::now() + Duration::minutes(minutes)
}

pub fn hours_ago(hours: i64) -> DateTime<Local> {
    Local::now() - Duration::hours(hours)
}

pub fn hours_from_now(hours: i64) -> DateTime<Local> {
    Local::now() + Duration::hours(hours)
}

pub fn days_ago(days: i64) -> NaiveDate {
    Local::now().date_naive() - Duration::days(days)
}

pub fn days_from_now(days: i64) -> NaiveDate {
    Local::now().date_naive() + Duration::days(days)
}

pub fn weeks_ago(weeks: i64) -> NaiveDate {
    Local::now().date_naive() - Duration::weeks(weeks)
}

pub fn weeks_from_now(weeks: i64) -> NaiveDate {
    Local::now().date_naive() + Duration::weeks(weeks)
}

pub fn yesterday() -> NaiveDate {
    days_ago(1)
}

pub fn tomorrow() -> NaiveDate {
    days_from_now(1)
}

pub fn months_ago(months: u32) -> NaiveDate {
    Local::now().date_naive() - Months::new(months)
}

pub fn months_from_now(months: u32) -> NaiveDate {
    Local::now().date_naive() + Months::new(months)
}

pub fn years_ago(years: u32) -> NaiveDate {
    months_ago(years * 12)
}

pub fn years_from_now(years: u32) -> NaiveDate {
    months_from_now(years * 12)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Local;

    #[test]
    fn test_seconds_ago_returns_correct_datetime() {
        let expected_before = Local::now() - Duration::seconds(3);
        let result = seconds_ago(3);
        let expected_after = Local::now() - Duration::seconds(3);

        assert!(result >= expected_before);
        assert!(result <= expected_after);
    }

    #[test]
    fn test_seconds_ago_with_zero_returns_now() {
        let before = Local::now();
        let result = seconds_ago(0);
        let after = Local::now();

        assert!(result >= before);
        assert!(result <= after);
    }

    #[test]
    fn test_seconds_ago_negative_equals_seconds_from_now() {
        assert_eq!(seconds_ago(-3).date_naive(), seconds_from_now(3).date_naive());
    }

    #[test]
    fn test_seconds_from_now_returns_correct_datetime() {
        let expected_before = Local::now() + Duration::seconds(3);
        let result = seconds_from_now(3);
        let expected_after = Local::now() + Duration::seconds(3);

        assert!(result >= expected_before);
        assert!(result <= expected_after);
    }

    #[test]
    fn test_seconds_from_now_with_zero_returns_now() {
        let before = Local::now();
        let result = seconds_from_now(0);
        let after = Local::now();

        assert!(result >= before);
        assert!(result <= after);
    }

    #[test]
    fn test_seconds_from_now_negative_equals_seconds_ago() {
        assert_eq!(seconds_from_now(-3).date_naive(), seconds_ago(3).date_naive());
    }

    #[test]
    fn test_minutes_ago_returns_correct_datetime() {
        let expected_before = Local::now() - Duration::minutes(3);
        let result = minutes_ago(3);
        let expected_after = Local::now() - Duration::minutes(3);

        assert!(result >= expected_before);
        assert!(result <= expected_after);
    }

    #[test]
    fn test_minutes_ago_with_zero_returns_now() {
        let before = Local::now();
        let result = minutes_ago(0);
        let after = Local::now();

        assert!(result >= before);
        assert!(result <= after);
    }

    #[test]
    fn test_minutes_ago_negative_equals_minutes_from_now() {
        assert_eq!(minutes_ago(-3).date_naive(), minutes_from_now(3).date_naive());
    }

    #[test]
    fn test_minutes_from_now_returns_correct_datetime() {
        let expected_before = Local::now() + Duration::minutes(3);
        let result = minutes_from_now(3);
        let expected_after = Local::now() + Duration::minutes(3);

        assert!(result >= expected_before);
        assert!(result <= expected_after);
    }

    #[test]
    fn test_minutes_from_now_with_zero_returns_now() {
        let before = Local::now();
        let result = minutes_from_now(0);
        let after = Local::now();

        assert!(result >= before);
        assert!(result <= after);
    }

    #[test]
    fn test_minutes_from_now_negative_equals_minutes_ago() {
        assert_eq!(minutes_from_now(-3).date_naive(), minutes_ago(3).date_naive());
    }

    #[test]
    fn test_hours_ago_negative_equals_hours_from_now() {
        assert_eq!(hours_ago(-3).date_naive(), hours_from_now(3).date_naive());
    }

    #[test]
    fn test_hours_from_now_returns_correct_datetime() {
        let expected_before = Local::now() + Duration::hours(3);
        let result = hours_from_now(3);
        let expected_after = Local::now() + Duration::hours(3);

        assert!(result >= expected_before);
        assert!(result <= expected_after);
    }

    #[test]
    fn test_hours_from_now_with_zero_returns_now() {
        let before = Local::now();
        let result = hours_from_now(0);
        let after = Local::now();

        assert!(result >= before);
        assert!(result <= after);
    }

    #[test]
    fn test_hours_from_now_negative_equals_hours_ago() {
        assert_eq!(hours_from_now(-3).date_naive(), hours_ago(3).date_naive());
    }

    #[test]
    fn test_days_ago_returns_correct_date() {
        let date = days_ago(3);
        let expected = Local::now().date_naive() - Duration::days(3);
        assert_eq!(date, expected);
    }

    #[test]
    fn test_days_ago_with_zero_returns_today() {
        assert_eq!(days_ago(0), Local::now().date_naive());
    }

    #[test]
    fn test_days_ago_negative_equals_days_from_now() {
        assert_eq!(days_ago(-3), days_from_now(3));
    }

    #[test]
    fn test_days_from_now_returns_correct_date() {
        let date = days_from_now(3);
        let expected = Local::now().date_naive() + Duration::days(3);
        assert_eq!(date, expected);
    }

    #[test]
    fn test_days_from_now_with_zero_returns_today() {
        assert_eq!(days_from_now(0), Local::now().date_naive());
    }

    #[test]
    fn test_days_from_now_negative_equals_days_ago() {
        assert_eq!(days_from_now(-3), days_ago(3));
    }

    #[test]
    fn test_weeks_ago_returns_correct_date() {
        let date = weeks_ago(2);
        let expected = Local::now().date_naive() - Duration::weeks(2);
        assert_eq!(date, expected);
    }

    #[test]
    fn test_weeks_ago_with_zero_returns_today() {
        assert_eq!(weeks_ago(0), Local::now().date_naive());
    }

    #[test]
    fn test_weeks_ago_negative_equals_weeks_from_now() {
        assert_eq!(weeks_ago(-2), weeks_from_now(2));
    }

    #[test]
    fn test_weeks_from_now_returns_correct_date() {
        let date = weeks_from_now(2);
        let expected = Local::now().date_naive() + Duration::weeks(2);
        assert_eq!(date, expected);
    }

    #[test]
    fn test_weeks_from_now_with_zero_returns_today() {
        assert_eq!(weeks_from_now(0), Local::now().date_naive());
    }

    #[test]
    fn test_weeks_from_now_negative_equals_weeks_ago() {
        assert_eq!(weeks_from_now(-2), weeks_ago(2));
    }

    #[test]
    fn test_yesterday_returns_previous_date() {
        let date = yesterday();
        let expected = Local::now().date_naive() - Duration::days(1);
        assert_eq!(date, expected);
    }

    #[test]
    fn test_tomorrow_returns_next_date() {
        let date = tomorrow();
        let expected = Local::now().date_naive() + Duration::days(1);
        assert_eq!(date, expected);
    }

    #[test]
    fn test_months_ago_returns_correct_date() {
        let date = months_ago(2);
        let expected = Local::now().date_naive() - Months::new(2);
        assert_eq!(date, expected);
    }

    #[test]
    fn test_months_ago_with_zero_returns_today() {
        assert_eq!(months_ago(0), Local::now().date_naive());
    }

    #[test]
    fn test_months_from_now_returns_correct_date() {
        let date = months_from_now(2);
        let expected = Local::now().date_naive() + Months::new(2);
        assert_eq!(date, expected);
    }

    #[test]
    fn test_months_from_now_with_zero_returns_today() {
        assert_eq!(months_from_now(0), Local::now().date_naive());
    }

    #[test]
    fn test_years_ago_returns_correct_date() {
        let date = years_ago(2);
        let expected = months_ago(24);
        assert_eq!(date, expected);
    }

    #[test]
    fn test_years_ago_with_zero_returns_today() {
        assert_eq!(years_ago(0), Local::now().date_naive());
    }

    #[test]
    fn test_years_from_now_returns_correct_date() {
        let date = years_from_now(2);
        let expected = months_from_now(24);
        assert_eq!(date, expected);
    }

    #[test]
    fn test_years_from_now_with_zero_returns_today() {
        assert_eq!(years_from_now(0), Local::now().date_naive());
    }
}
