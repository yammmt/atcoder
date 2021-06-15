// 26min
// 茶色…？ C で出たらかなりパニクりそう

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        an: [i64; n],
        lrvq: [(usize, usize, i64); q],
    }

    let mut diff_from_prev = vec![0; n];
    for i in 1..n {
        diff_from_prev[i] = an[i] - an[i - 1];
    }
    let mut ans = 0;
    diff_from_prev.iter().for_each(|d| ans += d.abs());

    for lrv in &lrvq {
        // println!("{:?}", diff_from_prev);
        if lrv.0 > 1 {
            ans -= diff_from_prev[lrv.0 - 1].abs();
            diff_from_prev[lrv.0 - 1] += lrv.2;
            ans += diff_from_prev[lrv.0 - 1].abs();
        }
        if lrv.1 < n {
            ans -= diff_from_prev[lrv.1].abs();
            diff_from_prev[lrv.1] -= lrv.2;
            ans += diff_from_prev[lrv.1].abs();
        }
        println!("{}", ans);
    }
}
