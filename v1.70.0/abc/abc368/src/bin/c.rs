use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        hn: [usize; n],
    }

    // 苦手

    // 一度に与えるダメージは
    // 1, 1, 3, 1, 1, 3, ...
    // 開始時の合計ターンを tt として, tt を 3 で割った余りが
    // - 1 のとき, 1, 6, 11, ...
    // - 2 のとき, 2, 7, 12, ...
    // - 0 のとき, 5, 10, 15, ...
    // 1 週で 5 のダメージを与えるところからサイクルを検出する
    // 1, 1, 3, ... のループになるよう開始位置を揃える

    let mut t = 0usize;
    for mut h in hn {
        let cycle = h / 5;
        t += cycle * 3;
        h -= 5 * cycle;
        while h > 0 {
            match t % 3 {
                2 => h -= h.min(3),
                _ => h -= 1,
            }
            t += 1;
        }
    }

    println!("{t}");
}
