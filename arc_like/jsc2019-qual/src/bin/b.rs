// :fu:

// https://qiita.com/vain0x/items/c9166707c81697198998

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        an: [u16; n],
    }
    let d = 10u64.pow(9) + 7;

    let mut first = 0u64;
    for i in 0..n {
        for j in i + 1..n {
            if an[i] > an[j] {
                first += 1;
            }
        }
    }

    let mut second = 0i64;
    for i in 0..n {
        for j in 0..n {
            if an[i] > an[j] {
                second += 1;
            }
        }
    }

    // println!("f: {}", first);
    // println!("s: {}", second);
    // return;

    let kc2 = ((k as u64 * (k as u64- 1) / 2) % d * second as u64) % d;
    let mut ans = (k as u64 * first) % d;
    ans = (ans + kc2) % d;
    println!("{}", ans);
}
