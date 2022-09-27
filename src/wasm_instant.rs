use std::{time::Duration, ops::{Add, AddAssign, Sub, SubAssign}};


use js_sys::Date;
use ordered_float::NotNan;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instant(NotNan<f64>);


impl Instant {
	/// 返回与“现在”相对应的瞬间。
	pub fn now () -> Instant {
		Instant(unsafe { NotNan::new_unchecked(Date::now()) })
	}

	/// 返回自创建此瞬间以来经过的时间量。
	/// # panic
	/// 如果当前时间早于该时刻，则此函数可能会出现恐慌，如果Instant综合生成，则可能会发生这种情况。
	pub fn elapsed(&self) -> Duration {
		Duration::from_nanos(((Date::now() - *self.0) * 1000000.0) as u64)
	}

	/// 返回从另一时刻到这一时刻所经过的时间量，如果该时刻晚于这一时刻，则返回零持续时间。
	/// # panic
	/// 以前，如果earlier晚于，我们会惊慌失措self。目前，此方法已饱和以遵循标准库的行为。在某些情况下，未来的版本可能会重新引入恐慌。
	pub fn duration_since(&self, earlier: Instant) -> Duration {
		let r = self.0 - earlier.0;
		if *r < 0.0 {
			Duration::from_nanos(0)
		} else {
			Duration::from_nanos(((*r) * 1000000.0) as u64)
		}
	}

	/// 返回从另一时刻到这一时刻所经过的时间量，如果该时刻晚于这一时刻，则返回无。
	pub fn checked_duration_since(&self, earlier: Instant) -> Option<Duration> {
		let r = self.0 - earlier.0;
		if *r < 0.0 {
			None
		} else {
			Some(Duration::from_nanos(((*r) * 1000000.0) as u64))
		}
	}

	/// 返回从另一时刻到这一时刻所经过的时间量，如果该时刻晚于这一时刻，则返回零持续时间。
	pub fn saturating_duration_since(&self, earlier: Instant) -> Duration {
		let r = self.0 - earlier.0;
		if *r < 0.0 {
			Duration::from_nanos(0)
		} else {
			Duration::from_nanos(((*r) * 1000000.0) as u64)
		}
	}

	pub fn checked_add(&self, duration: Duration) -> Option<Instant> {
		Some(Self(self.0 + (rhs.as_secs_f64() * 1000.0)))
	}

	pub fn checked_sub(&self, duration: Duration) -> Option<Instant> {
		Some(Self(self.0 - (rhs.as_secs_f64() * 1000.0)))
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

