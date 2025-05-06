use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    const MOD: usize = 1_000_000_007;
    input! {
        mut h: usize,
        r: usize,
        grr: [[usize; r]; r],
    }

    let matrix_mul = |a: &Vec<Vec<usize>>, b: &Vec<Vec<usize>>| {
        let nn = a.len();
        let mut ret = vec![vec![0; nn]; nn];
        for i in 0..nn {
            for j in 0..nn {
                for k in 0..nn {
                    ret[i][j] += a[i][k] * b[k][j];
                    ret[i][j] %= MOD;
                }
            }
        }
        ret
    };

    // 部屋の動き方は階に依らず同じだから行列累乗で高速化できる
    let mut mrr = vec![vec![0; r]; r];

    for s in 0..r {
        // dp[S][i]: 既に訪れた部屋の集合 S, 最後に訪れた部屋 i であるような状態の最小コスト
        // 最小である意味とは？ => 同じ部屋を通らないため
        let mut dp = vec![vec![0; r]; 1 << r];
        // 初期地点は 1 から 1 マス動いたところとする, これは戻って来る状態を表現するため, のはず
        dp[1 << s][s] = 1;

        // 貰う DP
        for ss in 0..(1 << r) {
            for i in 0..r {
                if ss & (1 << i) == 0 {
                    continue;
                }

                for j in 0..r {
                    if i == j || ss & (1 << j) == 0 {
                        continue;
                    }

                    // 貰う DP だから j, i の順
                    if grr[j][i] == 1 {
                        dp[ss][i] += dp[ss & !(1 << i)][j];
                        dp[ss][i] %= MOD;
                    }
                }
            }
        }
        for t in 0..r {
            for ss in 0..(1 << r) {
                mrr[s][t] += dp[ss][t];
                mrr[s][t] %= MOD;
            }
        }
    }

    // 初期値は単位ベクトル: 各頂点に至るパスが一つしかないため
    let mut ans = vec![vec![0; r]; r];
    for i in 0..r {
        ans[i][i] = 1;
    }
    while h > 0 {
        if h % 2 == 1 {
            ans = matrix_mul(&ans, &mrr);
        }
        mrr = matrix_mul(&mrr, &mrr);
        h /= 2;
    }

    println!("{}", ans[0][0]);
}
