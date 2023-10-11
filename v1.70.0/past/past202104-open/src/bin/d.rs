use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        an: [i64; n],
    }

    let mut ans = an.iter().take(k).sum::<i64>();
    for i in 0..n - k + 1 {
        println!("{ans}");
        ans -= an[i];
        if i != n - k {
            ans += an[k + i];
        }
    }
}
