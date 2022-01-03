use std::{
    thread,
    time::{Duration, Instant},
};

fn main() {
    let start = Instant::now();

    let handler1 = thread::spawn(|| {
        let pause = Duration::from_millis(300);
        thread::sleep(pause.clone())
    });

    let handler2 = thread::spawn(|| {
        let pause = Duration::from_millis(300);
        thread::sleep(pause);
    });

    handler1.join().unwrap();
    handler2.join().unwrap();

    let finish = Instant::now();

    println!("{:02?}", finish.duration_since(start));
}
