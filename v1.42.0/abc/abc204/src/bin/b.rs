use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [i64; n],
    }

    let mut ans = 0;
    for a in &an {
        ans += (*a - 10).max(0);
    }

    println!("{}", ans);
}
