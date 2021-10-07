use regex::Regex;
use std::time::Duration;

enum TimeUnit {
    SECOND,
    MINUTE,
    HOUR,
    INVALID,
}

pub fn parse_time(text: &str) -> Duration {
    // Time parsing rules
    // By default, assume minutes
    // Valid suffixes (case insensitive, no space allowed)
    // s|sec|second|seconds
    // m|min|minute|minutes
    // h|hour|hours

    // TODO: Make this a static?
    let time_pattern: Regex = Regex::new(r"(?P<value>[0-9]+)(?P<unit>[a-zA-Z]*)").unwrap();

    if let Some(time_pattern_match) = time_pattern.captures(text.trim()) {
        let number_value = (&time_pattern_match["value"]).parse::<u64>();
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

        if let Ok(time_value) = number_value {
            match unit {
                TimeUnit::SECOND => Duration::from_secs(time_value),
                TimeUnit::MINUTE => Duration::from_secs(time_value * 60),
                TimeUnit::HOUR => Duration::from_secs(time_value * 60 * 60),
                TimeUnit::INVALID => Duration::from_secs(0),
            }
        } else {
            Duration::from_secs(0)
        }
    } else {
        Duration::from_secs(0)
    }
}
