// :fu: 21-07 これも教育的

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: i64,
        b: i64,
        hn: [i64; n],
    }

    // Priority Queue を使って愚直に最高体力を狙い撃ちすれば良いが TLE
    let mut pass = *hn.iter().max().unwrap();
    let mut fail = 0;
    while pass - fail > 1 {
        let mid = (pass + fail) / 2;
        let mut a_atk = 0;
        for h in &hn {
            a_atk += ((*h - mid * b + a - b - 1) / (a - b)).max(0);
        }
        if a_atk <= mid {
            pass = mid;
        } else {
            fail = mid;
        }
    }

    println!("{}", pass);
}
