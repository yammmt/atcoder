use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    }
    let mut ans = 1;
    for _ in 0..(a - b) {
        ans *= 32;
    }
    println!("{}", ans);
}
