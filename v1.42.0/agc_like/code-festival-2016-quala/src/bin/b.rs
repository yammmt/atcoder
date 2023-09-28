// 2min

use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [usize; n],
    }

    let mut ans = 0;
    for (i, a) in an.iter().enumerate() {
        if an[*a - 1] == i + 1 {
            ans += 1;
        }
    }
    println!("{}", ans / 2);
}
