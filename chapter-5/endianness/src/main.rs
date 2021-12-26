// 内部的には std::intrinsics を呼び出しているみたい
use std::mem::transmute;

fn main() {
    let big_endian: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
    let little_endian: [u8; 4] = [0xDD, 0xCC, 0xBB, 0xAA];

    println!(
        "{:08b} {:08b} {:08b} {:08b}",
        big_endian[0], big_endian[1], big_endian[2], big_endian[3]
    );
    println!(
        "{:08b} {:08b} {:08b} {:08b}",
        little_endian[0], little_endian[1], little_endian[2], little_endian[3]
    );

    let a: i32 = unsafe { transmute(big_endian) };
    let b: i32 = unsafe { transmute(little_endian) };

    println!("{} vs {}", a, b);
    println!("{:032b} vs {:032b}", a, b);
}
