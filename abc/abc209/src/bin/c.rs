// :fu: :fu:

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut cn: [u64; n],
    }
    let d = 1_000_000_007;
    cn.sort();

    let mut ans = 1u64;
    for (i, c) in cn.iter().enumerate() {
        if i as u64 + 1 > *c {
            println!("0");
            return;
        }

        ans = (ans * (*c - i as u64)) % d;
    }

    println!("{}", ans);
}
