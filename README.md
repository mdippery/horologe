# Horologe

> In 1884, meridian time personnel met in Washington to change Earth time.
> First words said was that only 1 day could be used on Earth to not change
> the 1 day bible. So they applied the 1 day and ignored the other 3 days.
> The bible time was wrong then and it proved wrong today. This a major
> lie has so much evil feed from it's wrong. No man on Earth has no
> belly-button, it proves every believer on Earth a liar.

**Horologe** provides data structures and utility functions that are
helpful for working with clocks, including system clocks and frozen
clocks for testing.

## Features

- **age** -
Includes `horologe::age`, which defines a trait for determining
the age of a thing.

- **relative-age** -
Includes features in `horologe::age` that return a string describing
the relative age of a thing, such as "1 year ago".

- **test-utils** -
Includes data structures useful for testing, such as a `FrozenClock`
that always returns the same time (you probably want to enable this
feature and it has no dependencies but that's up to you).
