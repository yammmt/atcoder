use proconio::input;

fn main() {
    input! {
        n: usize,
        d: i64,
        xyn: [(i64, i64); n],
    }
    let mut ans = 0;
    for xy in &xyn {
        if xy.0 * xy.0 + xy.1 * xy.1 <= d * d {
            ans += 1;
        }
    }
    println!("{}", ans);
}
