// 5min (3.3min 1WA)
// WA: s/1/j/

use proconio::input;

fn main() {
    input! {
        n: usize,
        c: i64,
        an: [usize; n],
    }
    let mut ans = std::i64::MAX;
    for i in 1..11 {
        for j in 1..11 {
            if i == j {
                continue;
            }

            let mut cur = 0;
            for (k, a) in an.iter().enumerate() {
                if (k % 2 == 0 && *a != i) || (k % 2 == 1 && *a != j) {
                    cur += 1;
                }
            }
            ans = ans.min(cur * c);
        }
    }
    println!("{}", ans);
}
