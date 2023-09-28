use proconio::input;

fn main() {
    input! {
        _n: usize,
        m: usize,
        lrm: [(usize, usize); m],
    }

    let mut lmax = 0;
    let mut rmin = std::usize::MAX / 2;
    for lr in &lrm {
        lmax = lmax.max(lr.0);
        rmin = rmin.min(lr.1);
    }

    println!(
        "{}",
        if lmax > rmin {
            0
        } else {
            rmin - lmax + 1
        }
    );
}
