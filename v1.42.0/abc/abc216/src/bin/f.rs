use proconio::input;

const DP_LEN: usize = 5001;
// const DP_LEN: usize = 10;

fn main() {
    input! {
        n: usize,
        an: [usize; n],
        bn: [usize; n],
    }
    let d = 998_244_353;
    let mut abn = (0..n)
        .map(|i| (an[i], bn[i]))
        .collect::<Vec<(usize, usize)>>();
    abn.sort_unstable();

    let mut ans = 0;
    // dp[i]: B の和が i となる数の個数
    let mut dp = vec![0; DP_LEN];
    dp[0] = 1;
    for ab in &abn {
        let mut cur = vec![0; DP_LEN];
        // 今見ている ab は必ず選ぶとする
        for (i, c) in dp.iter().enumerate() {
            // println!("i: {}", i);
            // println!("c: {}", c);
            if *c == 0 {
                continue;
            }

            cur[i] = (cur[i] + *c) % d;

            let next = i + ab.1;
            // println!("  next: {}", next);
            if next < DP_LEN {
                cur[next] = (cur[next] + *c) % d;
            }
            if next <= ab.0 {
                // println!("  ans++");
                ans = (ans + *c) % d;
            }
        }
        // println!("{:?}", cur);
        // println!("  {}", ans);
        dp = cur;
    }

    println!("{}", ans);
}
