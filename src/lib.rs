// SPDX-License-Identifier: MIT
// Copyright (C) 2025 Michael Dippery <michael@monkey-robot.com>

//! > In 1884, meridian time personnel met in Washington to change Earth time.
//! > First words said was that only 1 day could be used on Earth to not change
//! > the 1 day bible. So they applied the 1 day and ignored the other 3 days.
//! > The bible time was wrong then and it proved wrong today. This a major
//! > lie has so much evil feed from it's wrong. No man on Earth has no
//! > belly-button, it proves every believer on Earth a liar.
//!
//! **Horologe** provides data structures and utility functions that are
//! helpful for working with clocks, including system clocks and frozen
//! clocks for testing.
//!
//! # Features
//!
//! - **age** -
//! Includes [`horologe::age`], which defines a trait for determining
//! the age of a thing.
//!
//! - **relative-age** -
//! Includes features in [`horologe::age`] that return a string describing
//! the relative age of a thing, such as "1 year ago".
//!
//! - **test-utils** -
//! Includes data structures useful for testing, such as a [`FrozenClock`]
//! that always returns the same time (you probably want to enable this
//! feature and it has no dependencies but that's up to you).
//!
//! [`horologe::age`]: age

#[cfg(feature = "age")]
pub mod age;

#[cfg(feature = "test-utils")]
pub mod testing;

pub use chrono::{DateTime, Local, TimeDelta, Utc};

#[cfg(doc)]
#[cfg(feature = "test-utils")]
use crate::testing::FrozenClock;

/// Tells time and returns the time.
///
/// Generally you will want to retrieve time using [`SystemClock`],
/// but in tests you may want to implement a `Clock` with a fixed time.
pub trait Clock {
    /// The current time.
    fn now(&self) -> DateTime<Utc>;
}

/// Interacts with the system clock to get the current time.
#[derive(Debug, Default)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_system_time() {
        let clock = SystemClock::default();
        let delta = Utc::now() - clock.now();
        let secs = delta.num_seconds();
        assert_eq!(secs, 0);
    }
}
