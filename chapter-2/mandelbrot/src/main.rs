use num::complex::Complex; // num クレートの complexからComplex型をインポート
// 出力空間(行と列の格子)と、マンデルブロ集合を囲む範囲
// (原点(0, 0)に近い連続領域)の間で変換を行う関数
fn calculate_mandelbrot(
    max_iters: usize, // 繰り返しの上限に達するまで外れなかった値は集合内とみなす
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    width: usize,
    height: usize,
) -> Vec<Vec<usize>> {
    // 各行からデータを格納するためのコンテナを作る
    let mut rows: Vec<_> = Vec::with_capacity(width);
    // 行ごとの繰り返しによって、出力を行単位で出力
    for img_y in 0..height {
        let mut row: Vec<usize> = Vec::with_capacity(width);
        for img_x in 0..width {
            let x_percent = (img_x as f64 / width as f64);
            let y_percent = (img_y as f64 / height as f64);
            // 出力でカバーされる空間の比率を計算し、それをサーチ空間内の点に変換する
            let cx = x_min + (x_max - x_min) * x_percent;
            let cy = y_min + (y_max - y_min) * y_percent;
            let escaped_at = mandelbrot_at_point(cx, cy, max_iters);
            row.push(escaped_at);
        }

        rows.push(row);
    }
    // そういえばreturn xxx;というより xxx の方が使われているのかな？うーん...
    rows
}

// 個々の要素(標準出力に出力される個々の行と列の組み合わせ)について呼び出される関数
fn mandelbrot_at_point(
    cx: f64,
    cy: f64,
    max_iters: usize
) -> usize {
    // 原点の複素数zを計算。実数部(re)と仮数部(im)が0.0
    // 実部と虚部じゃね？仮数部は浮動小数点数に使う用語のような
    // 正誤表に書いてあった
    let mut z = Complex{re: 0.0, im: 0.0};
    let c = Complex::new(cx, cy);
    for i in 0..=max_iters {
        if z.norm() > 2.0 { 
            return i;
        }
        z = z * z + c;
    }
    // return を書くのか書かないのかどっちやねん
    return max_iters;
}

fn render_mandelbrot(escape_vals: Vec<Vec<usize>>) {
    for row in escape_vals {
        let mut line = String::with_capacity(row.len());
        for column in row {
            let val = match column {
                0..=2 => ' ',
                3..=5 => '.',
                6..=10 => '?',
                11..=30 => '*',
                31..=100 => '+',
                101..=200 => 'x',
                201..=400 => '$',
                401..=700 => '#',
                _ => '%'
            };
            line.push(val);
        }
        println!("{}", line);
    }
}

fn main() {
    let mandelbrot = calculate_mandelbrot(1000, -2.0, 1.0, -1.0, 1.0, 80, 24);

    render_mandelbrot(mandelbrot);
}