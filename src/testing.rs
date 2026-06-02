// SPDX-License-Identifier: MIT
// Copyright (C) 2025 Michael Dippery <michael@monkey-robot.com>

//! Utilities for tests involving time.

use crate::{Clock, DateTime, Error, Utc};

#[cfg(doc)]
use chrono::ParseError;

/// A clock that always returns the same time.
///
/// This is useful for "freezing time" in unit tests. In your tests, call
/// [`FrozenClock::default()`] to create a clock that always returns the
/// time specified in [`FrozenClock::default_datetime`]. This clock's notion
/// of "now" (as returned by [`FrozenClock::now()`]) is always the same and
/// always in the past.
#[derive(Debug)]
pub struct FrozenClock {
    datetime: DateTime<Utc>,
}

impl FrozenClock {
    const DEFAULT_DATETIME: &'static str = "2025-05-23T10:13:00-07:00";

    /// The default datetime, used when constructing default frozen clocks.
    ///
    /// This date is guaranteed to always be in the past.
    pub fn default_datetime() -> DateTime<Utc> {
        DateTime::parse_from_rfc3339(Self::DEFAULT_DATETIME.as_ref())
            .expect(&format!("invalid default date: {}", Self::DEFAULT_DATETIME))
            .with_timezone(&Utc)
    }

    /// Creates a new frozen clock that always returns the given `datetime`
    /// as "now".
    pub fn new(datetime: DateTime<Utc>) -> Self {
        FrozenClock { datetime }
    }
}

impl TryFrom<String> for FrozenClock {
    type Error = Error;

    /// Creates a new frozen clock that always returns the given `datetime`
    /// as "now".
    ///
    /// Returns a `Result` wrapping a chrono [`ParseError`] if `datetime`
    /// cannot be parsed.
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl TryFrom<&str> for FrozenClock {
    type Error = Error;

    /// Creates a new frozen clock that always returns the given `datetime`
    /// as "now".
    ///
    /// Returns a `Result` wrapping a chrono [`ParseError`] if `datetime`
    /// cannot be parsed.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let datetime = DateTime::parse_from_rfc3339(value)?.with_timezone(&Utc);
        Ok(Self::new(datetime))
    }
}

impl Clock for FrozenClock {
    fn now(&self) -> DateTime<Utc> {
        self.datetime
    }
}

impl Default for FrozenClock {
    /// Always returns a datetime specified by [`FrozenClock::default_datetime`]
    /// for "now".
    fn default() -> Self {
        Self::new(Self::default_datetime())
    }
}

#[cfg(test)]
mod tests {
    use super::FrozenClock;
    use crate::{Clock, Error, SystemClock};
    use chrono::{Datelike, Timelike};

    #[test]
    fn it_parses_a_datetime_string() -> Result<(), Error> {
        let s = "2025-12-25T12:01:02-08:00";
        let dt = FrozenClock::try_from(String::from(s))?.now();
        assert_eq!(dt.month(), 12);
        assert_eq!(dt.day(), 25);
        assert_eq!(dt.year(), 2025);
        assert_eq!(dt.hour(), 20);
        assert_eq!(dt.minute(), 1);
        assert_eq!(dt.second(), 2);
        Ok(())
    }

    #[test]
    fn it_parses_a_datetime_str() -> Result<(), Error> {
        let s = "2025-12-25T12:01:02-08:00";
        let dt = FrozenClock::try_from(s)?.now();
        assert_eq!(dt.month(), 12);
        assert_eq!(dt.day(), 25);
        assert_eq!(dt.year(), 2025);
        assert_eq!(dt.hour(), 20);
        assert_eq!(dt.minute(), 1);
        assert_eq!(dt.second(), 2);
        Ok(())
    }

    #[test]
    fn it_returns_an_error_if_there_is_an_invalid_string() {
        let s = "2025-12-32T12:01:02-08:00";
        let c = FrozenClock::try_from(s);
        assert!(c.is_err());
    }

    #[test]
    fn it_returns_a_default_date_in_the_past() {
        let now = SystemClock::default().now();
        let dt = FrozenClock::default().now();
        assert!(dt < now);
    }
}
