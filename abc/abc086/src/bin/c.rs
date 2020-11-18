// 6min

use proconio::input;

fn main() {
    input! {
        n: usize,
        txyn: [(i64, i64, i64); n],
    }

    for i in 0..n {
        let prev = if i == 0 {
            (0, 0, 0)
        } else {
            txyn[i - 1]
        };
        let time_passed = txyn[i].0 - prev.0;
        let d = (txyn[i].1 - prev.1).abs() + (txyn[i].2 - prev.2).abs();
        if d > time_passed || (time_passed - d) % 2 != 0 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
