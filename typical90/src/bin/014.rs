use proconio::input;

fn main() {
    input! {
        n: usize,
        mut an: [i64; n],
        mut bn: [i64; n],
    }
    an.sort_unstable();
    bn.sort_unstable();
    let mut ans = 0;
    for i in 0..n {
        ans += (an[i] - bn[i]).abs();
    }
    println!("{}", ans);
}
