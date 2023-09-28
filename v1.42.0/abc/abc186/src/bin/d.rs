use proconio::input;

fn main() {
    input! {
        n: usize,
        mut an: [i64; n],
    }
    an.sort_unstable();
    an.reverse();

    let mut cur = an.iter().sum::<i64>();
    let mut ans = 0;
    for i in 0..n {
        cur -= an[i];
        ans += (n - i - 1) as i64 * an[i] - cur;
    }

    println!("{}", ans);
}
