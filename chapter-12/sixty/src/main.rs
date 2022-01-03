use std::{process, thread::sleep, time::Duration};

fn main() {
    let delay = Duration::from_secs(1);

    let pid = process::id();
    println!("{}", pid);

    for i in 1..=60 {
        sleep(delay);
        println!(". {}", i);
    }
}
