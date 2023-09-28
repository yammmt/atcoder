// 数問 嫌

use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! {
        x: Bytes,
    }
    let mut vn: Vec<usize> = x.iter().map(|n| (n - b'0') as usize).collect();
    vn.reverse();
    let vn = vn;

    // 愚直に文字列で繰り上がり含め計算すると
    // 最大入力が 5x10^5 であり桁数二乗のオーダーでは TLE
    let mut cusum: usize = vn.iter().sum();
    let mut carry_up = 0;
    let mut ans = vec![];
    for n in &vn {
        let cur = cusum + carry_up;
        ans.push(cur % 10);
        // println!("ans: {:?}", ans);
        // println!("  cusum: {}", cusum);
        carry_up = cur / 10;
        cusum -= *n;
    }
    while carry_up > 0 {
        ans.push(carry_up % 10);
        carry_up /= 10;
    }

    ans.reverse();
    for a in ans {
        print!("{}", a);
    }
    println!();
}
