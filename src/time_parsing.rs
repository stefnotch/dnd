use chrono::Duration;
use regex::Regex;

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
        let number_value = (&time_pattern_match["value"]).parse::<i64>();
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
                TimeUnit::SECOND => Duration::seconds(time_value),
                TimeUnit::MINUTE => Duration::seconds(time_value * 60),
                TimeUnit::HOUR => Duration::seconds(time_value * 60 * 60),
                TimeUnit::INVALID => Duration::seconds(0),
            }
        } else {
            Duration::seconds(0)
        }
    } else {
        Duration::seconds(0)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_seconds() {
        assert_eq!(parse_time("0s"), Duration::seconds(0));
        assert_eq!(parse_time("1s"), Duration::seconds(1));
        assert_eq!(parse_time("3s"), Duration::seconds(3));
        assert_eq!(parse_time("9s"), Duration::seconds(9));
        assert_eq!(parse_time("13s"), Duration::seconds(13));
        assert_eq!(parse_time("130s"), Duration::seconds(130));
        assert_eq!(parse_time("342s"), Duration::seconds(342));
        assert_eq!(parse_time("130sec"), Duration::seconds(130));
        assert_eq!(parse_time("130second"), Duration::seconds(130));
        assert_eq!(parse_time("130seconds"), Duration::seconds(130));
    }

    #[test]
    fn test_parse_invalid_seconds() {
        assert_eq!(parse_time("-1s"), Duration::seconds(0));
        assert_eq!(parse_time("-10s"), Duration::seconds(0));
        assert_eq!(parse_time("xas"), Duration::seconds(0));
        assert_eq!(parse_time("1 s"), Duration::seconds(0));
        assert_eq!(parse_time("130se"), Duration::seconds(0));
        assert_eq!(parse_time("130s3x"), Duration::seconds(0));
    }

    #[test]
    fn test_parse_minutes() {
        assert_eq!(parse_time("0m"), Duration::seconds(0 * 60));
        assert_eq!(parse_time("9m"), Duration::seconds(9 * 60));
        assert_eq!(parse_time("13"), Duration::seconds(13 * 60));
        assert_eq!(parse_time("24min"), Duration::seconds(24 * 60));
        assert_eq!(parse_time("24minutes"), Duration::seconds(24 * 60));
    }

    #[test]
    fn test_parse_invalid_minutes() {
        assert_eq!(parse_time("-10min"), Duration::seconds(0));
        assert_eq!(parse_time("x"), Duration::seconds(0));
        assert_eq!(parse_time("1 minute"), Duration::seconds(0));
    }

    #[test]
    fn test_parse_hours() {
        assert_eq!(parse_time("9h"), Duration::seconds(9 * 60 * 60));
        assert_eq!(parse_time("24hours"), Duration::seconds(24 * 60 * 60));
    }

    #[test]
    fn test_parse_invalid_hours() {
        assert_eq!(parse_time("-10hours"), Duration::seconds(0));
    }
}
