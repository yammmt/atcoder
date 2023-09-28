use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [usize; n],
    }
    let d = 998_244_353;

    let mut dp = vec![0u64; 10];
    dp[(an[0] + an[1]) % 10] += 1;
    dp[(an[0] * an[1]) % 10] += 1;
    for i in 2..n {
        let mut cur = vec![0; 10];
        for j in 0..10 {
            if dp[j] == 0 {
                continue;
            }

            let next_add = (j + an[i]) % 10;
            cur[next_add] = (cur[next_add] + dp[j]) % d;
            let next_mul = (j * an[i]) % 10;
            cur[next_mul] = (cur[next_mul] + dp[j]) % d;
        }
        dp = cur;
    }

    for a in &dp {
        println!("{}", a);
    }
}
