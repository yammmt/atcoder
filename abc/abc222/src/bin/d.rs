use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [usize; n],
        bn: [usize; n],
    }
    let d = 998_244_353;

    // O(N^2)
    // dp[i]: 今の c_i の値が i である場合は何通りあるか
    let mut dp = vec![0; 3001];
    dp[0] = 1;
    for i in 0..n {
        let mut cur = vec![0; 3001];
        let mut ptrn = 0;
        for dpp in dp.iter().take(an[i]) {
            ptrn = (ptrn + *dpp) % d;
        }

        for c in an[i]..bn[i] + 1 {
            ptrn = (ptrn + dp[c]) % d;
            cur[c] = ptrn;
        }

        dp = cur;
    }

    let mut ans = 0;
    for dpp in &dp {
        ans = (ans + *dpp) % d;
    }
    println!("{}", ans);
}
