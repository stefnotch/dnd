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
    let time_pattern: Regex = Regex::new(r"^(?P<value>[0-9]+)(?P<unit>[a-zA-Z]*)$").unwrap();

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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_seconds() {
        assert_eq!(parse_time("0s"), Duration::from_secs(0));
        assert_eq!(parse_time("1s"), Duration::from_secs(1));
        assert_eq!(parse_time("3s"), Duration::from_secs(3));
        assert_eq!(parse_time("9s"), Duration::from_secs(9));
        assert_eq!(parse_time("13s"), Duration::from_secs(13));
        assert_eq!(parse_time("130s"), Duration::from_secs(130));
        assert_eq!(parse_time("342s"), Duration::from_secs(342));
    }

    #[test]
    fn test_parse_invalid_seconds() {
        assert_eq!(parse_time("-1s"), Duration::from_secs(0));
        assert_eq!(parse_time("0s"), Duration::from_secs(0));
        assert_eq!(parse_time("-10s"), Duration::from_secs(0));
        assert_eq!(parse_time("xas"), Duration::from_secs(0));
        assert_eq!(parse_time("1 s"), Duration::from_secs(0));
    }
}
