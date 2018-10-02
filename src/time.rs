/// All time in clockwork-mage is measured in centiseconds; i.e.,
/// `Timestamp(123)` means '1.23 seconds since simulation start',
/// `Duration(4.56)` means '4.56 seconds', etc.

use std::ops::*;

/// Time since simulation start.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Timestamp(pub i32);

/// Difference between two `Timestamp`s.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Duration(pub i32);

impl Add<Duration> for Timestamp {
    type Output = Timestamp;
    fn add(self, duration: Duration) -> Timestamp {
        Timestamp(self.0 + duration.0)
    }
}

impl AddAssign<Duration> for Timestamp {
    fn add_assign(&mut self, duration: Duration) {
        *self = *self + duration
    }
}

impl Sub for Timestamp {
    type Output = Duration;
    fn sub(self, timestamp: Timestamp) -> Duration {
        Duration(self.0 - timestamp.0)
    }
}


impl Sub<Duration> for Timestamp {
    type Output = Timestamp;
    fn sub(self, duration: Duration) -> Timestamp {
        Timestamp(self.0 - duration.0)
    }
}

impl Sub for Duration {
    type Output = Duration;
    fn sub(self, duration: Duration) -> Duration {
        Duration(self.0 - duration.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(Timestamp(2) + Duration(3), Timestamp(5));
    }

    fn test_add_assign() {
        let mut t = Timestamp(5);
        t += Duration(4);
        assert_eq!(t, Timestamp(9));
    }

    #[test]
    fn test_timestamp_minus_duration() {
        assert_eq!(Timestamp(8) - Duration(3), Timestamp(5));
    }

    #[test]
    fn test_timestamp_minus_timestamp() {
        assert_eq!(Timestamp(8) - Timestamp(2), Duration(6));
    }

    #[test]
    fn test_duration_minus_duration() {
        assert_eq!(Duration(9) - Duration(6), Duration(3));
    }
}
