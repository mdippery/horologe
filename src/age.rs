// SPDX-License-Identifier: MIT
// Copyright (C) 2025 Michael Dippery <michael@monkey-robot.com>

//! Age calculations.

use crate::{Clock, DateTime, Local, TimeDelta, Utc};

#[cfg(feature = "relative-age")]
use regex::Regex;

#[cfg(feature = "relative-age")]
use relativetime::NegativeRelativeTime;

#[cfg(feature = "relative-age")]
use std::time::Duration;

#[cfg(doc)]
use crate::SystemClock;

/// Marks a thing that has a notion of its age.
pub trait HasAge {
    /// The date the item was created, in UTC.
    fn created_utc(&self) -> DateTime<Utc>;

    /// The date the item was created, in local time.
    fn created_local(&self) -> DateTime<Local> {
        self.created_utc().with_timezone(&Local)
    }

    /// The age of the thing.
    ///
    /// `clock` is a source of time from which the age can be derived.
    /// Generally [`SystemClock::default()`] is used.
    fn age<C: Clock>(&self, clock: &C) -> TimeDelta {
        let birthday = self.created_utc();
        clock.now() - birthday
    }

    /// The age of the thing, relative to the current time, as a
    /// human-readable string.
    ///
    /// `clock` is a source of time from which the age can be derived.
    /// Generally [`SystemClock::default()`] is used.
    #[cfg(feature = "relative-age")]
    fn relative_age<C: Clock>(&self, clock: &C) -> String {
        let age = self.age(clock).as_seconds_f64();
        let d = Duration::from_secs(age.trunc() as u64);
        let s = d.to_relative_in_past();

        // The relativetime crate sometimes prints things like "1 months ago".
        // Unfortunately, the crate is no longer updated and isn't even on
        // GitHub anymore, so it's not likely to be updated any time soon,
        // so let's just hack around the bug here until we have a better fix.
        let re = Regex::new("^1 (?<unit>[a-z]+)s ago$").unwrap();
        re.replace(&s, "1 $unit ago").to_string()
    }
}

#[cfg(test)]
#[cfg(all(feature = "test-utils", feature="relative-age"))]
mod tests {
    use super::*;
    use crate::testing::FrozenClock;

    #[derive(Debug)]
    struct ThingWithAge {
        created_utc: DateTime<Utc>,
    }

    impl ThingWithAge {
        pub fn new(timestamp: i64) -> Self {
            let created_utc = DateTime::from_timestamp(timestamp, 0).unwrap();
            Self { created_utc }
        }
    }

    impl HasAge for ThingWithAge {
        fn created_utc(&self) -> DateTime<Utc> {
            self.created_utc
        }
    }

    #[test]
    fn it_returns_its_age() {
        let clock = FrozenClock::default();
        let thing = ThingWithAge::new(1349074800);
        assert_eq!(thing.age(&clock).num_seconds(), 398945580);
    }

    #[test]
    fn it_returns_its_age_as_a_relative_string() {
        let clock = FrozenClock::default();
        let thing = ThingWithAge::new(1349074800);
        assert_eq!(thing.relative_age(&clock), "13 years ago");
    }

    #[test]
    fn it_correctly_formats_singular_time_units() {
        let datetime = DateTime::parse_from_rfc3339("2025-05-28T10:51:00-07:00")
            .expect("could not parse timestamp")
            .with_timezone(&Utc);
        let clock = FrozenClock::new(datetime);
        let thing = ThingWithAge::new(1744177355);
        assert_eq!(thing.relative_age(&clock), "1 month ago");
    }

    #[test]
    fn it_correctly_formats_singular_time_units_with_indefinite_articles() {
        let datetime = DateTime::parse_from_rfc3339("2025-05-28T10:51:00-07:00")
            .expect("could not parse timestamp")
            .with_timezone(&Utc);
        let clock = FrozenClock::new(datetime);
        let thing = ThingWithAge::new(1744481059);
        assert_eq!(thing.relative_age(&clock), "a month ago");
    }
}
