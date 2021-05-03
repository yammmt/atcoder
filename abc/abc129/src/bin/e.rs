// 桁 DP
// WA: 答えが `i32` 型になっていた

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        l: Chars,
    }
    let d = 1_000_000_007;

    let mut two_table = vec![0; l.len() + 1];
    two_table[0] = 1;
    for i in 1..l.len() + 1 {
        two_table[i] = (two_table[i - 1] * 2) % d;
    }

    let mut three_table = vec![0; l.len() + 1];
    three_table[0] = 1;
    for i in 1..l.len() + 1 {
        three_table[i] = (three_table[i - 1] * 3) % d;
    }

    let mut ans = 0u64;
    let mut one_num = 0;
    for (i, c) in l.iter().enumerate() {
        // println!("i: {}", i);
        if *c == '1' {
            // 上から i 桁目を 0 とすればそこから後ろの桁はなんでも良い
            // 既出の bit 1 を分配しつつ a/b の続く桁が 0/0, 0/1, 1/0 の三通り
            ans = (ans + (two_table[one_num] * three_table[l.len() - i - 1]) % d) % d;
            // println!("  ans: {}", ans);
            one_num += 1;
        }

        if i == l.len() - 1 {
            // 最終桁でありこれまでに出た 1 を a/b どちらに入れるか計算して足す
            ans = (ans + two_table[one_num]) % d;
            // println!("  ans: {}", ans);
            break;
        }
    }

    println!("{}", ans);
}
