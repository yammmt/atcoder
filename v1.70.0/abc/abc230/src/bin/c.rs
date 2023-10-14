// これが C つらい

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: isize,
        b: isize,
        p: usize,
        q: usize,
        r: usize,
        s: usize,
    }

    // 1 <= b-k <= n
    // 1-b <= -k <= n-b
    // b-1 >= k >= b-n

    for i in p - 1..q {
        let di = (i as isize - a + 1).abs();
        for j in r - 1..s {
            let dj = (j as isize - b + 1).abs();
            print!(
                "{}",
                if di == dj {
                    '#'
                } else {
                    '.'
                }
            );
        }
        println!();
    }
}
