use proconio::input;

fn main() {
    input! {
        _n: usize,
        m: usize,
        mut abm: [(usize, usize); m],
    }
    abm.sort_by(|a, b| {
        if a.1 != b.1 {
            a.1.cmp(&b.1)
        } else {
            a.0.cmp(&b.0)
        }
    });
    // println!("{:?}", abm);

    let mut ans = 0;
    let mut detached = 0;
    for ab in &abm {
        if ab.0 >= detached {
            // println!("{}", ab.1);
            detached = ab.1;
            ans += 1;
        }
    }

    println!("{}", ans);
}
