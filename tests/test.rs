use std::thread;
use std::time::{Duration, Instant};

use pi_time::{run_micros, start_secs, HPClock};

#[test]
fn test() {
    println!("start time: {:?}, run time: {:?}", start_secs(), run_micros());

    let mut count = 0usize;
    for index in 0..100000000 {
        if index % 2 == 0 {
            count += 1;
        }
    }

    println!("start time: {:?}, run time: {:?}, count: {:?}", start_secs(), run_micros(), count);
}

#[test]
fn test_high_performance_clock_by_no_wasm32() {
    let clock = HPClock::new();

    let mut clock0 = clock.clone();
    let now = Duration::default();
    let join0 = thread::spawn(move || {
        for _ in 0..10 {
            let new_now = clock0.elapsed();
            println!("!!!!!!new_now0: {:?}", new_now);
            assert!(clock0.elapsed() >= now);
        }
    });

    let mut clock1 = clock.clone();
    let now = Duration::default();
    let join1 = thread::spawn(move || {
        clock1.tick();
        let new_now = clock1.elapsed();
        assert!(new_now != now);

        for _ in 0..10 {
            println!("!!!!!!new_now1: {:?}", clock1.elapsed());
            assert_eq!(clock1.elapsed(), new_now);
        }
    });

    join0.join();
    join1.join();
}