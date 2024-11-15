

#[cfg(not(target_arch = "wasm32"))]
pub mod clock {
    use std::ptr::null_mut;
    use std::time::{Duration, Instant};
    use std::sync::{Arc,
                    atomic::{AtomicPtr, Ordering}};

    ///
    /// 高性能时钟，时间精度为百微秒级
    ///
    pub struct HPClock(Arc<InnerHPClock>);
    
    impl Clone for HPClock {
        fn clone(&self) -> Self {
            HPClock(self.0.clone())
        }
    }

    impl HPClock {
        /// 构建一个高性能时钟
        pub fn new() -> Self {
            let clock = Instant::now();
            let interval = clock.elapsed();
            let inner = InnerHPClock {
                interval: AtomicPtr::new(Box::into_raw(Box::new(interval))),
                clock: AtomicPtr::new(Box::into_raw(Box::new(clock))),
            };

            HPClock(Arc::new(inner))
        }

        /// 获取当前时钟流逝的时间
        pub fn elapsed(&self) -> Duration {
            let raw = self.0.interval.load(Ordering::Acquire);
            let boxed = unsafe { Box::from_raw(raw) };
            let r = *boxed;
            Box::into_raw(boxed);

            r
        }

        /// 向前推动当前时钟
        pub fn tick(&mut self) {
            //获取当前时钟
            let current = self
                .0
                .clock
                .swap(null_mut(), Ordering::AcqRel);
            if current.is_null() {
                //正在推动当前时钟，则忽略
                return;
            }
            let boxed = unsafe { Box::from_raw(current) };
            let new = boxed.elapsed();

            //重置当前时钟的流逝时间
            let old = self
                .0
                .interval
                .swap(Box::into_raw(Box::new(new)),
                       Ordering::AcqRel);
            unsafe { let _ = Box::from_raw(old); }

            //归还当前时钟
            self
                .0
                .clock
                .store(Box::into_raw(boxed), Ordering::Release);
        }
    }

    // 内部高性能时钟
    struct InnerHPClock {
        interval:   AtomicPtr<Duration>,
        clock:      AtomicPtr<Instant>,
    }
}

#[cfg(target_arch = "wasm32")]
pub mod clock {
    use std::rc::Rc;
    use std::cell::RefCell;
    use std::ptr::null_mut;
    use std::time::Duration;
    use std::sync::atomic::{AtomicPtr, Ordering};

    use crate::wasm_instant::Instant;

    ///
    /// 高性能时钟，时间精度为毫秒级
    ///
    pub struct HPClock(Rc<RefCell<InnerHPClock>>);

    unsafe impl Send for HPClock {}
    unsafe impl Sync for HPClock {}

    impl Clone for HPClock {
        fn clone(&self) -> Self {
            HPClock(self.0.clone())
        }
    }

    impl HPClock {
        /// 构建一个高性能时钟
        pub fn new() -> Self {
            let clock = Instant::now();
            let interval = clock.elapsed();
            let inner = InnerHPClock {
                interval,
                clock,
            };

            HPClock(Rc::new(RefCell::new(inner)))
        }

        /// 获取当前时钟流逝的时间
        pub fn elapsed(&self) -> Duration {
            self.0.borrow().interval
        }

        /// 向前推动当前时钟
        pub fn tick(&mut self) {
            let interval = self
                .0
                .borrow()
                .clock
                .elapsed();
            self.0.borrow_mut().interval = interval;
        }
    }

    // 内部高性能时钟
    struct InnerHPClock {
        interval:   Duration,
        clock:      Instant,
    }
}