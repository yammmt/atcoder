use proconio::input;

fn main() {
    input! {
        n: usize,
        xvn: [(f64, String); n],
    }
    let mut ans = 0.0;
    for xv in &xvn {
        ans += if &*(xv.1) == "JPY" {
            xv.0
        } else {
            xv.0 * 380_000.0
        };
    }
    println!("{}", ans);
}
