use std::{
    thread,
    time::{Duration, Instant},
};

fn main() {
    let start = Instant::now();

    let handler = thread::spawn(|| {
        let pause = Duration::from_millis(300);
        thread::sleep(pause.clone())
    });

    handler.join().unwrap();

    let finish = Instant::now();

    println!("{:02?}", finish.duration_since(start));
}
