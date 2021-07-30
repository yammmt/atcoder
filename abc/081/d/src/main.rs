// :fu: :fu: :fu: 21-07 ARC

use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [i64; n],
    }

    // 貪欲で条件を満たす？
    let mut amax = std::i64::MIN;
    let mut amin = std::i64::MAX;
    let mut amaxi = 0;
    let mut amini = 0;
    for (i, a) in an.iter().enumerate() {
        if *a > amax {
            amax = *a;
            amaxi = i;
        }

        if *a < amin {
            amin = *a;
            amini = i;
        }
    }

    let mut ans = vec![];
    if amax.abs() >= amin.abs() {
        for i in 0..n {
            ans.push((amaxi + 1, i + 1));
        }
        for i in 1..n {
            ans.push((i, i + 1));
        }
    } else {
        for i in 0..n {
            ans.push((amini + 1, i + 1));
        }
        for i in (1..n).rev() {
            ans.push((i + 1, i));
        }
    }

    println!("{}", ans.len());
    for a in &ans {
        println!("{} {}", a.0, a.1);
    }
}
