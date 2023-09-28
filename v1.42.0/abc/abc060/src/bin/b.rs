// 9.5min

use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
    }

    let mut appeared = vec![false; b as usize];
    let mut cur = a;
    loop {
        if cur % b == c {
            println!("YES");
            return;
        } else if appeared[(cur % b) as usize] {
            println!("NO");
            return;
        }

        appeared[(cur % b) as usize] = true;
        cur += a;
    }
}
