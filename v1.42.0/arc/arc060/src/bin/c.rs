// 28.5min 同じ札を何度も使わない

use proconio::input;

#[allow(clippy::needless_range_loop)]
fn main() {
    input! {
        n: usize,
        a: usize,
        xn: [usize; n],
    }

    // score[i][j]: i 枚のカードで合計が j となる選び方の個数
    let mut score = vec![vec![0u64; n * a + 1]; n + 1];
    score[0][0] = 1;

    for x in &xn {
        let mut cur_score = vec![vec![0u64; n * a + 1]; n + 1];
        for i in 0..n {
            for j in 0..n * a {
                if j + *x > n * a {
                    break;
                } else if score[i][j] == 0 {
                    continue;
                }

                cur_score[i + 1][j + *x] += score[i][j];
            }
        }

        for i in 0..n + 1 {
            for j in 0..n * a + 1 {
                score[i][j] += cur_score[i][j];
            }
        }
    }

    let mut ans = 0;
    for i in 1..n + 1 {
        ans += score[i][i * a];
    }

    println!("{}", ans);
}
