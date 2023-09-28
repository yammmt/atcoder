use proconio::input;

fn main() {
    input! {
        n: usize,
        pn: [f64; n],
    }

    // i 枚投げ終わった時点で表が j 枚出る確率
    let mut dp = vec![vec![0.0; n + 1]; n + 1];
    dp[0][0] = 1.0;
    for (i, p) in pn.iter().enumerate() {
        // println!("i: {}", i);
        for j in 0..i + 1 {
            // println!("  j: {}", j);
            // 自身を投げた結果が表
            dp[i + 1][j + 1] += dp[i][j] * *p;
            // 自身を投げた結果が裏
            dp[i + 1][j] += dp[i][j] * (1.0 - *p);
        }
    }
    // println!("{:?}", dp);

    println!(
        "{}",
        dp[n].iter().skip(n / 2 + 1).sum::<f64>()
    );
}
