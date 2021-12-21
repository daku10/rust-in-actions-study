use std::time::{Duration, Instant};

fn main() {
    let mut count = 1;
    let time_limit = Duration::new(1, 0); // 1秒間を表すDurationを作る
    let start = Instant::now(); // システムクロックから開始時刻Instantを首藤
    // 2つのInstantの源さんはDurationを返す
    while (Instant::now() - start) < time_limit {
        count += 1;
    }
    println!("{}", count);
}
