// 37min

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }
    let d = 10u64.pow(9) + 7;

    let mut vc = vec![0; 26];
    for c in &s {
        vc[(*c as u8 - b'a') as usize] += 1;
    }
    let mut ans = 1;
    for i in &vc {
        if *i == 0 {
            continue;
        }

        ans = (ans * (*i + 1)) % d;
    }
    ans = (ans + d - 1) % d;
    println!("{}", ans);
}
