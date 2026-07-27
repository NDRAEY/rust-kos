use crate::time::Duration;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Instant(Duration);

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct SystemTime(Duration);

pub const UNIX_EPOCH: SystemTime = SystemTime(Duration::from_secs(0));

// Is that a performance clock?
impl Instant {
    pub fn now() -> Instant {
        // TODO: This implementation of Instant is just a stub.
        // When I found how to convert KolibriOS' performance clock, I edit this func.
        
        let date = crate::sys::pal::api::date();
        let time = crate::sys::pal::api::time();

        let dt = crate::sys::pal::time::DateTime {
            date, time
        };

        let unix = dt.to_unix();

        Instant(Duration::from_secs(unix))
    }

    pub fn checked_sub_instant(&self, other: &Instant) -> Option<Duration> {
        self.0.checked_sub(other.0)
    }

    pub fn checked_add_duration(&self, other: &Duration) -> Option<Instant> {
        Some(Instant(self.0.checked_add(*other)?))
    }

    pub fn checked_sub_duration(&self, other: &Duration) -> Option<Instant> {
        Some(Instant(self.0.checked_sub(*other)?))
    }
}

impl SystemTime {
    pub const MAX: SystemTime = SystemTime(Duration::MAX);

    pub const MIN: SystemTime = SystemTime(Duration::ZERO);

    pub fn now() -> SystemTime {
        let date = crate::sys::pal::api::date();
        let time = crate::sys::pal::api::time();

        let dt = crate::sys::pal::time::DateTime {
            date, time
        };

        let unix = dt.to_unix();

        SystemTime(Duration::from_secs(unix))
    }   

    pub fn sub_time(&self, other: &SystemTime) -> Result<Duration, Duration> {
        self.0.checked_sub(other.0).ok_or_else(|| other.0 - self.0)
    }

    pub fn checked_add_duration(&self, other: &Duration) -> Option<SystemTime> {
        Some(SystemTime(self.0.checked_add(*other)?))
    }

    pub fn checked_sub_duration(&self, other: &Duration) -> Option<SystemTime> {
        Some(SystemTime(self.0.checked_sub(*other)?))
    }
}
