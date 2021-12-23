#![allow(unused_variables)]

type File = String;

fn open(f: &mut File) -> bool {
    true // この関数は、常に成功するものとする
}

fn close(f: &mut File) -> bool {
    true // この関数は、常に成功するものとする
}

#[allow(dead_code)]
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
    unimplemented!();
}

fn main() {
    let mut f1 = File::from("f1.txt");
    open(&mut f1);
    // read(f1, vec![]);
    close(&mut f1);
}
