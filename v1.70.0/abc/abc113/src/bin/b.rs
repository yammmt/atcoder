use proconio::input;

fn main() {
    input! {
        n: usize,
        t: f64,
        a: f64,
        hn: [f64; n],
    }

    let mut ans = 0;
    let mut score = f64::MAX / 2.0;

    for (i, h) in hn.iter().enumerate() {
        let cur = (a - (t - *h * 0.006)).abs();
        if cur < score {
            score = cur;
            ans = i + 1;
        }
    }

    println!("{ans}");
}
