use proconio::input;

fn main() {
    input! {
        n: usize,
        a: u64,
        b: u64,
        xn: [u64; n],
    }

    let mut ans = 0;
    for i in 1..n {
        ans += (a * (xn[i] - xn[i - 1])).min(b);
    }
    println!("{}", ans);
}
