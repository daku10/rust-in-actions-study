use std::{
    thread,
    time::{Duration, Instant},
};

fn main() {
    for n in 1..1001 {
        let mut handlers: Vec<thread::JoinHandle<()>> = Vec::with_capacity(n);

        let start = Instant::now();
        for _m in 0..n {
            let handle = thread::spawn(|| {
                let pause = Duration::from_millis(20);
                thread::sleep(pause);
            });
            handlers.push(handle);
        }

        while let Some(handle) = handlers.pop() {
            handle.join();
        }

        let finish = Instant::now();
        println!("{}\t{:02?}", n, finish.duration_since(start));
    }
}
