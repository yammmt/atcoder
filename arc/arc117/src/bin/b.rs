use proconio::input;

fn main() {
    input! {
        n: usize,
        mut an: [u64; n],
    }
    let d = 1_000_000_007u64;
    an.sort();
    an.reverse();
    an.push(0);

    let mut ans = 1;
    for i in 0..n {
        ans = (ans * (an[i] - an[i + 1] + 1)) % d;
    }
    println!("{}", ans);
}
