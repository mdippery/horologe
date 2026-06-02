// SPDX-License-Identifier: MIT
// Copyright (C) 2025 Michael Dippery <michael@monkey-robot.com>

#![doc = include_str!("../README.md")]

#[cfg(feature = "age")]
pub mod age;
#[cfg(feature = "test-utils")]
pub mod testing;

pub use chrono::{DateTime, Local, TimeDelta, Utc};
use thiserror::Error;

#[cfg(feature = "test-utils")]
use chrono::ParseError;

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

/// Encapsulates all errors thrown by the library.
#[derive(Debug, Error)]
pub enum Error {
    #[cfg(feature = "test-utils")]
    #[error("could not parse datetime string")]
    DateTimeFormat(#[from] ParseError),
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
