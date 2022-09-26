use std::{time::Duration, ops::{Add, AddAssign, Sub, SubAssign}};


use js_sys::Date;
use ordered_float::NotNan;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instant(NotNan<f64>);


impl Instant {
	pub fn now () -> Instant {
		Instant(unsafe { NotNan::new_unchecked(Date::now()) })
	}

	pub fn elapsed(&self) -> Duration {
		Duration::from_nanos(((Date::now() - *self.0) * 1000000.0) as u64)
	}
}

impl Add<Duration> for Instant {
    type Output = Instant;

    fn add(self, rhs: Duration) -> Self::Output {
		Instant(self.0 + (rhs.as_secs_f64() * 1000.0))
    }
}

impl AddAssign<Duration> for Instant {
    fn add_assign(&mut self, rhs: Duration) {
        self.0 = self.0 + (rhs.as_secs_f64() * 1000.0);
    }
}

impl Sub<Duration> for Instant {
    type Output = Instant;

    fn sub(self, rhs: Duration) -> Self::Output {
        Self(self.0 - (rhs.as_secs_f64() * 1000.0))
    }
}

impl SubAssign<Duration> for Instant {
    fn sub_assign(&mut self, rhs: Duration) {
        self.0 = self.0 - (rhs.as_secs_f64() * 1000.0);
    }
}

impl Sub<Instant> for Instant {
    type Output = Duration;

    fn sub(self, rhs: Instant) -> Self::Output {
        Duration::from_nanos(((*self.0 - *rhs.0) * 1000000.0) as u64)
    }
}

