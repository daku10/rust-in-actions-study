fn main() {
    let search_term = "picture";
    let quote = "\
Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books.
What do we seek through millions of pages?";
    // lines()が返す引用文のイテレータで、各テキスト行が反復処理される
    // 改行の解釈は実機OSの規約が使われる
   
    for (i, line) in quote.lines().enumerate() {
        let line_num = i + 1;
        if line.contains(search_term) {
            println!("{}: {}", line_num, line);
        }
    }
}
