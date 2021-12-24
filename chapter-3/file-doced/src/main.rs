//! ファイル群を断崖的にシミュレートする

/// fileは、アクセス可能なファイルを意味する
#[derive(Debug)]
pub struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    /// 新規ファイルは空とみなすが、ファイル名は必須
    ///
    /// # 例
    ///
    /// ```
    /// let f = File::new("f1.txt");
    /// ```
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }

    /// ファイルの長さ(バイト数を返す)
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// ファイル名を返す
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

fn main() {
    let f1 = File::new("f1.txt");

    let f1_name = f1.name();
    let f1_length = f1.len();

    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);
}
