use std::thread;
use std::time::{Duration, Instant};

use pi_time::{run_micros, start_secs};

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