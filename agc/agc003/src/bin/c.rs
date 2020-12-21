// 22min

use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [i64; n],
    }

    let mut van = Vec::with_capacity(n);
    for (i, a) in an.iter().enumerate() {
        van.push((a, i as i64));
    }
    van.sort_unstable();
    let mut diff_odd = 0;
    for (i, a) in van.iter().enumerate() {
        if (i as i64 - a.1).abs() % 2 == 1 {
            diff_odd += 1;
        }
    }
    println!("{}", (diff_odd + 1) / 2);
}
