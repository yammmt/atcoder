use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        abn: [(usize, usize); n],
    }

    // (金額, 集計対象？)
    let coins = [
        (500, false),
        (100, false),
        (50, true),
        (10, false),
        (5, true),
        (1, false),
    ];
    let mut ans = 0;
    for (a, b) in abn {
        let mut change = b - a;
        for c in &coins {
            let returns = change / c.0;
            change -= c.0 * returns;
            if c.1 {
                ans += returns;
            }
        }
    }

    println!("{ans}");
}
