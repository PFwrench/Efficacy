use super::errors::CliError;
use chrono::{DateTime, Datelike, Duration, Local, TimeZone, Utc, Weekday};

pub const DATE_FMT: &str = "%F %T";

fn calculate_day_difference(weekday: Weekday) -> i64 {
    let current_day = Local::now().weekday().number_from_monday() as i64;
    let due_day = weekday.number_from_monday() as i64;
    (due_day - current_day + 7) % 7
}

pub fn weekday_to_due_date(weekday: Weekday) -> DateTime<Utc> {
    Local::today()
        .checked_add_signed(Duration::days(calculate_day_difference(weekday)))
        .unwrap()
        .and_hms(8, 0, 0)
        .with_timezone(&Utc)
}

pub fn string_to_weekday(string: &str) -> Result<Weekday, CliError> {
    match &string.to_lowercase()[..] {
        "mon" | "monday" => Ok(Weekday::Mon),
        "tue" | "tues" | "tu" | "tuesday" => Ok(Weekday::Tue),
        "wed" | "wednesday" => Ok(Weekday::Wed),
        "thu" | "thurs" | "thur" | "thursday" => Ok(Weekday::Thu),
        "fri" | "friday" => Ok(Weekday::Fri),
        "sat" | "saturday" | "sa" => Ok(Weekday::Sat),
        "sun" | "sunday" | "su" => Ok(Weekday::Sun),
        _ => Err(CliError::ParsingError),
    }
}

pub fn string_to_due_date(string: &str) -> Result<DateTime<Utc>, CliError> {
    Ok(match Local.datetime_from_str(string, DATE_FMT) {
        Ok(d) => d.with_timezone(&Utc),
        Err(_) => return Err(CliError::ParsingError),
    })
}
