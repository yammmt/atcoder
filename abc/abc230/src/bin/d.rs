use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        mut lrn: [(usize, usize); n],
    }
    lrn.sort_unstable_by(|a, b| {
        if a.1 != b.1 {
            a.1.cmp(&b.1)
        } else {
            a.0.cmp(&b.0)
        }
    });

    let mut ans = 1;
    let mut panch_reached = lrn[0].1 + d - 1;
    for lr in &lrn {
        if lr.0 > panch_reached {
            panch_reached = lr.1 + d - 1;
            ans += 1;
        }
    }

    println!("{}", ans);
}
