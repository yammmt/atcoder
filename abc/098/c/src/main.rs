// WA: インデックス

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut wsum = vec![0; n + 1];
    let mut esum = vec![0; n + 1];
    for (i, c) in s.iter().enumerate() {
        if *c == 'W' {
            wsum[i + 1] = wsum[i] + 1;
            esum[i + 1] = esum[i];
        } else {
            wsum[i + 1] = wsum[i];
            esum[i + 1] = esum[i] + 1;
        }
    }

    let mut ans = std::usize::MAX / 3;
    for i in 0..n {
        // 自分より左で W + 自分より右で E
        ans = ans.min(wsum[i] + esum[n] - esum[i + 1]);
    }

    println!("{}", ans);
}
