// :fu: 21-06 数問 (確率) 巨大数の割り算をなんとかする

use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        x: isize,
        y: isize,
    }
    let x = x.abs() as usize;
    let y = y.abs() as usize;

    if x % d != 0 || y % d != 0 {
        println!("0.0");
        return;
    }

    let required_x = x / d;
    let required_y = y / d;
    let vain_num = n - required_x - required_y;
    if vain_num % 2 != 0 {
        println!("0.0");
        return;
    }

    // N = 1,000 に対して nCr を 128bit 整数型で求めてもオーバーフロー
    let mut ncr = vec![vec![0.0; n + 2]; n + 2];
    ncr[0][0] = 1.0;
    for i in 0..n + 1 {
        for j in 0..i + 1 {
            ncr[i + 1][j] += ncr[i][j];
            ncr[i + 1][j + 1] += ncr[i][j];
        }
    }

    // 愚直なシミュレーションは最大 4^N (N <= 1,000) 点程を見ることになって TLE しそう
    // 全通りを一気に割ろうとしても分母が 4^N になって計算不可
    // 先に分子を求めてから N 回 4.0 で割ってやろうもと分子時点でオーバーフロー

    let mut ans = 0.0;
    // 最短経路と端数で積を取ると重複が発生して WA
    // x 方向に i 回余分に動くとする
    for i in 0..vain_num + 1 {
        if i % 2 == 1 {
            continue;
        }

        let x_move_num = required_x + i;
        let y_move_num = n - x_move_num;
        let x_up_num = required_x + (x_move_num - required_x) / 2;
        let y_up_num = required_y + (y_move_num - required_y) / 2;

        let mut cur = ncr[n][x_move_num];
        for _ in 0..n {
            cur /= 4.0;
        }

        // println!("i: {}", i);
        // println!("  x_move_num: {}", x_move_num);
        // println!("  y_move_num: {}", y_move_num);
        // println!("  ncr[{}][{}]", n, x_move_num);
        // println!("  ncr[{}][{}]", x_move_num, x_up_num);
        // println!("  ncr[{}][{}]", y_move_num, y_up_num);
        ans += cur * ncr[x_move_num][x_up_num] * ncr[y_move_num][y_up_num];
    }

    println!("{}", ans);
}
