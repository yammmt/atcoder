// :fu: :fu: :fu: 数問 (幾何) + 日本語
// ABC-C 問題史上最難易度では？ ABC188-E (diff1170) と正答率 2% しか変わらない
// super-ryuma と比べてもサンプルが弱く問題文の意図がわからないままだった

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        shw: [Chars; h],
    }
    let dir = [(0, 0), (0, 1), (1, 0), (1, 1)];

    let mut ans = 0;
    for i in 0..h - 1 {
        for j in 0..w - 1 {
            let mut blacks = 0;
            for d in &dir {
                if shw[i + d.0][j + d.1] == '#' {
                    blacks += 1;
                }
            }
            if blacks == 1 || blacks == 3 {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
