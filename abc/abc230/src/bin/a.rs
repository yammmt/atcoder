use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = n;
    if n > 41 {
        ans += 1;
    }
    println!("AGC{:03}", ans);
}
