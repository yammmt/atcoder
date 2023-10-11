// 最短時間かと思った

use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        t: usize,
        x: usize,
        abn: [(usize, usize); n],
    }

    if abn.iter().any(|&ab| ab.0 > t && ab.1 >= l) {
        println!("forever");
        return;
    }

    let mut overwork_time = 0;
    let mut ans = 0;
    for ab in &abn {
        let a = ab.0;
        let b = ab.1;
        if b < l {
            overwork_time = 0;
            ans += a;
        } else {
            if overwork_time + a > t {
                // ペナルティだけ別計算
                ans += t - overwork_time;
                ans += x;
                overwork_time = 0;
            }

            ans += a;
            if overwork_time + a == t {
                ans += x;
                overwork_time = 0;
            } else {
                overwork_time += a;
            }
        }
    }

    println!("{ans}");
}
