use std::time::Duration;

use regex::Regex;

const TIME_PATTERN: Regex = Regex::new(r"(?P<value>[-]?[0-9]+)(?P<unit>[a-zA-Z]*)").unwrap();

enum TimeUnit {
    SECOND,
    MINUTE,
    HOUR,
    INVALID,
}

pub fn parse_time(text: &str) -> Duration {
    if let Some(time_pattern_match) = TIME_PATTERN.captures(text.trim()) {
        let number_value = (&time_pattern_match["value"]).parse::<i32>();
        let unit = match (&time_pattern_match["unit"]).to_lowercase().as_str() {
            "s" | "sec" | "second" | "seconds" => TimeUnit::SECOND,
            "m" | "min" | "minute" | "minutes" => TimeUnit::MINUTE,
            "h" | "hr" | "hour" | "hours" => TimeUnit::HOUR,
            v => {
                if v.is_empty() {
                    TimeUnit::MINUTE
                } else {
                    TimeUnit::INVALID
                }
            }
        };

        if let Some(time_value) = number_value {
            Duration::from_secs(0)
        } else {
            Duration::from_secs(0)
        }
    } else {
        Duration::from_secs(0)
    }
}
