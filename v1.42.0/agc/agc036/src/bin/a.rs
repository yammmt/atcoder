// 緑前半とは嘘では

use proconio::input;

fn main() {
    input! {
        s: u64,
    }

    let d = (s / 10u64.pow(9) + 1).min(10u64.pow(9));
    let c = 10u64.pow(9) * d - s;
    println!("0 0 1000000000 1 {} {}", c, d);
}
