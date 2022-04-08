use std::{mem::MaybeUninit, time::Duration};

use libc::{
    clock_gettime,
    CLOCK_MONOTONIC,
    CLOCK_MONOTONIC_RAW,
    timespec,
};

fn get_time() -> timespec {
    let mut start = MaybeUninit::zeroed();

    clock_gettime(
        CLOCK_MONOTONIC,
        start.as_mut_ptr(),
    );

    unsafe { start.assume_init() }
}

fn main() {
    let start = get_time();

    println!("start: {}", start.tv_sec);

    std::thread::sleep(Duration::from_secs(60));

    let end = get_time();
    println!("end: {}", end.tv_sec);
}
