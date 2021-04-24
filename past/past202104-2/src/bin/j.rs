use proconio::input;

fn main() {
    input! {
        n: usize,
        c: f64,
        xyn: [(f64, f64); n],
    }

    let mut sumx = 0.0;
    for xy in &xyn {
        sumx += xy.0;
    }
    let avex = sumx / (n as f64);
    let mut ans = 0.0;
    for xy in &xyn {
        ans += (avex - xy.0) * (avex - xy.0) + (c - xy.1) * (c - xy.1);
    }
    println!("{}", ans);
}
