use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        ak: [Usize1; k],
        xyn: [(i64, i64); n],
    }

    let mut has_light = vec![false; n];
    for a in &ak {
        has_light[*a] = true;
    }

    let mut ans = 0i64;
    for i in 0..n {
        if has_light[i] {
            continue;
        }

        let mut cur_min = i64::MAX;
        for &j in &ak {
            let dx = xyn[i].0 - xyn[j].0;
            let dy = xyn[i].1 - xyn[j].1;
            cur_min = cur_min.min(dx * dx + dy * dy);
        }

        ans = ans.max(cur_min);
    }

    println!("{}", (ans as f64).sqrt());
}
