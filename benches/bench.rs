#![feature(test)]

extern crate test;

use test::Bencher;

use std::thread;
use std::time::Duration;

use crossbeam_channel::bounded;
#[cfg(feature = "high_performance")]
use pi_time::{HPClock, tick_clock, now_microsecond};

#[bench]
fn bench_high_performance_clock_elapsed_by_not_wasm32(b: &mut Bencher) {
    let clock = HPClock::new();
    let (sender0, receiver0) = bounded(8);

    let mut clock0 = clock.clone();
    let _ = thread::spawn(move || {
        loop {
            clock0.tick();
            thread::sleep(Duration::from_millis(16));
        }
    });

    thread::sleep(Duration::from_millis(1000));

    let clock1 = clock.clone();
    let sender0_copy = sender0.clone();
    let (sender1, receiver1) = bounded(1);
    let _ = thread::spawn(move || {
        loop {
            if let Ok(_) = receiver1.recv() {
                let mut r = Duration::default();
                for _ in 0..10000 {
                    r = clock1.elapsed();
                }
                let _ = sender0_copy.send(r);
            }
        }
    });

    let clock2 = clock.clone();
    let sender0_copy = sender0.clone();
    let (sender2, receiver2) = bounded(1);
    let _ = thread::spawn(move || {
        loop {
            if let Ok(_) = receiver2.recv() {
                let mut r = Duration::default();
                for _ in 0..10000 {
                    r = clock2.elapsed();
                }
                let _ = sender0_copy.send(r);
            }
        }
    });

    let clock3 = clock.clone();
    let sender0_copy = sender0.clone();
    let (sender3, receiver3) = bounded(1);
    let _ = thread::spawn(move || {
        loop {
            if let Ok(_) = receiver3.recv() {
                let mut r = Duration::default();
                for _ in 0..10000 {
                    r = clock3.elapsed();
                }
                let _ = sender0_copy.send(r);
            }
        }
    });

    let clock4 = clock.clone();
    let sender0_copy = sender0.clone();
    let (sender4, receiver4) = bounded(1);
    let _ = thread::spawn(move || {
        loop {
            if let Ok(_) = receiver4.recv() {
                let mut r = Duration::default();
                for _ in 0..10000 {
                    r = clock4.elapsed();
                }
                let _ = sender0_copy.send(r);
            }
        }
    });

    let clock5 = clock.clone();
    let sender0_copy = sender0.clone();
    let (sender5, receiver5) = bounded(1);
    let _ = thread::spawn(move || {
        loop {
            if let Ok(_) = receiver5.recv() {
                let mut r = Duration::default();
                for _ in 0..10000 {
                    r = clock5.elapsed();
                }
                let _ = sender0_copy.send(r);
            }
        }
    });

    let clock6 = clock.clone();
    let sender0_copy = sender0.clone();
    let (sender6, receiver6) = bounded(1);
    let _ = thread::spawn(move || {
        loop {
            if let Ok(_) = receiver6.recv() {
                let mut r = Duration::default();
                for _ in 0..10000 {
                    r = clock6.elapsed();
                }
                let _ = sender0_copy.send(r);
            }
        }
    });

    let clock7 = clock.clone();
    let sender0_copy = sender0.clone();
    let (sender7, receiver7) = bounded(1);
    let _ = thread::spawn(move || {
        loop {
            if let Ok(_) = receiver7.recv() {
                let mut r = Duration::default();
                for _ in 0..10000 {
                    r = clock7.elapsed();
                }
                let _ = sender0_copy.send(r);
            }
        }
    });

    let clock8 = clock.clone();
    let sender0_copy = sender0.clone();
    let (sender8, receiver8) = bounded(1);
    let _ = thread::spawn(move || {
        loop {
            if let Ok(_) = receiver8.recv() {
                let mut r = Duration::default();
                for _ in 0..10000 {
                    r = clock8.elapsed();
                }
                let _ = sender0_copy.send(r);
            }
        }
    });

    b.iter(move || {
        let _ = sender1.send(());
        let _ = sender2.send(());
        let _ = sender3.send(());
        let _ = sender4.send(());
        let _ = sender5.send(());
        let _ = sender6.send(());
        let _ = sender7.send(());
        let _ = sender8.send(());

        let mut count = 0;
        while count < 8 {
            match receiver0.recv() {
                Err(e) => panic!("Bench failed, reason: {:?}", e),
                Ok(r) => assert!(clock.elapsed() >= r && r > Duration::default()),
            }
            count += 1;
        }
    });
}

#[bench]
fn bench_high_performance_clock_tick_by_not_wasm32(b: &mut Bencher) {
    let clock = HPClock::new();
    let (sender0, receiver0) = bounded(8);

    let mut clock0 = clock.clone();
    let (sender1, receiver1) = bounded(1);
    let _ = thread::spawn(move || {
        loop {
            if let Ok(_) = receiver1.recv() {
                for _ in 0..10000 {
                    clock0.tick();
                }
                let _ = sender0.send(clock0.elapsed());
            }
        }
    });

    b.iter(move || {
        let _ = sender1.send(());

        match receiver0.recv() {
            Err(e) => panic!("Bench failed, reason: {:?}", e),
            Ok(r) => assert!(clock.elapsed() >= r && r > Duration::default()),
        }
    });
}

#[bench]
fn bench_high_performance_wrap_clock_tick_by_not_wasm32(b: &mut Bencher) {
    let (sender0, receiver0) = bounded(8);

    let (sender1, receiver1) = bounded(1);
    let _ = thread::spawn(move || {
        loop {
            if let Ok(_) = receiver1.recv() {
                for _ in 0..10000 {
                    tick_clock();
                }
                let _ = sender0.send(now_microsecond());
            }
        }
    });

    b.iter(move || {
        let _ = sender1.send(());

        match receiver0.recv() {
            Err(e) => panic!("Bench failed, reason: {:?}", e),
            Ok(r) => assert!(now_microsecond() >= r && r > 0),
        }
    });
}