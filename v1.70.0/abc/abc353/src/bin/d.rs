use proconio::fastout;
use proconio::input;

const MOD: usize = 998_244_353;

fn digit(mut n: usize) -> usize {
    let mut ret = 0;
    while n > 0 {
        ret += 1;
        n /= 10;
    }
    ret
}

#[fastout]
fn main() {
    input! {
        n: usize,
        an: [usize; n],
    }

    // mod がなければ, 最下位桁から一つずつ…としても時間間に合いそう

    // digit_num[i][j]: i 番目までの要素に j 桁の数がいくつあるか
    let mut digit_num = vec![vec![0; 11]; n + 1];
    for (i, &a) in an.iter().enumerate() {
        for j in 0..11 {
            digit_num[i + 1][j] = digit_num[i][j];
        }

        digit_num[i + 1][digit(a)] += 1;
    }

    let mut ans = 0usize;
    for (i, &a) in an.iter().enumerate() {
        // 自身より前の要素と組むと自身が下位
        ans += (a * i) % MOD;
        ans %= MOD;

        // 自身より後の要素と組むと自身が上位
        let mut a_upper = a;
        for j in 0..11 {
            // 最悪入力では 10^9 に 10^10 したものを 10^5 回足すことになる
            ans += (a_upper * (digit_num[n][j] - digit_num[i + 1][j])) % MOD;
            ans %= MOD;
            if j != 10 {
                a_upper *= 10;
                a_upper %= MOD;
            }
        }
    }

    println!("{ans}");
}
