fn main() {
    // needle(針)は、haystack(干草の山)に含まれる値をもつ変数
    // 何にも使われていないんだが... この後の例の使い回しだからかな
    let needle = 42;
    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

    for item in &haystack {
        // このmatch式は、変数に結びつけることが可能な値を返す
        let result = match item {
            // 見つけた! 42|132は、42と132の両方にマッチする
            42 | 132 => "hit!",
            // ワイルドカードの_は、何でもマッチするパターン
            _ => "miss",
        };

        if result == "hit!" {
            println!("{}: {}", item, result);
        }
    }
}
