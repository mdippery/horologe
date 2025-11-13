// SPDX-License-Identifier: MIT
// Copyright (C) 2025 Michael Dippery <michael@monkey-robot.com>

//! Utilities for tests involving time.

use crate::{Clock, DateTime, Utc};

/// A clock that always returns the same time.
///
/// This is useful for "freezing time" in unit tests.
/// ```
pub struct FrozenClock {
    datetime: DateTime<Utc>,
}

// TODO: Maybe a macro for implementing Default with a static string would be cool.

impl FrozenClock {
    /// Creates a new frozen clock that always returns the given `datetime`
    /// as "now".
    pub fn new(datetime: DateTime<Utc>) -> Self {
        FrozenClock { datetime }
    }

    /// Creates a new frozen clock that always returns the given `datetime`
    /// as "now".
    ///
    /// # Panics
    ///
    /// If `datetime` is not a valid time string.
    pub fn with_string(datetime: impl AsRef<String>) -> Self {
        let datetime = DateTime::parse_from_rfc3339(datetime.as_ref())
            // TODO: Probably should have better error handling, I dunno
            .expect("invalid date supplied")
            .with_timezone(&Utc);
        Self::new(datetime)
    }
}

impl Clock for FrozenClock {
    fn now(&self) -> DateTime<Utc> {
        self.datetime
    }
}
