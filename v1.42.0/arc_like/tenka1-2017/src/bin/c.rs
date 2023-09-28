// 40min 1WA
// WA: s/n/N/

// h が 3500 以下か否かはチェックしなくとも良い

use proconio::input;

fn main() {
    input! {
        N: u64,
    }

    for n in 1..3501 {
        for w in 1..3501 {
            if 4 * n * w <= N * w + N * n {
                continue;
            }

            if (N * n * w) % (4 * n * w - N * w - N * n) == 0 {
                println!(
                    "{} {} {}",
                    (N * n * w) / (4 * n * w - N * w - N * n),
                    n,
                    w
                );
                return;
            }
        }
    }
}
