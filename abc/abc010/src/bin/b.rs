// 手間取らない

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut an: [usize; n],
    }

    // mod 2 => 1
    // mod 3 => 0 or 1
    let mut ans = 0;
    for a in an.iter_mut() {
        while *a % 2 == 0 || *a % 3 == 2 {
            *a -= 1;
            ans += 1;
        }
    }

    println!("{}", ans);
}
