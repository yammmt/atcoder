// 二次元累積和の典型

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        chw: [[i64; w]; h],
    }
    let mut black = vec![vec![0; w]; h];
    let mut white = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            if i % 2 == j % 2 {
                black[i][j] = chw[i][j];
            } else {
                white[i][j] = chw[i][j];
            }
        }
    }

    let mut black_sum = vec![vec![0; w + 1]; h + 1];
    for i in 0..h {
        for j in 0..w {
            black_sum[i + 1][j + 1] =
                black_sum[i + 1][j] + black_sum[i][j + 1] - black_sum[i][j] + black[i][j];
        }
    }
    let black_sum = black_sum;

    let mut white_sum = vec![vec![0; w + 1]; h + 1];
    for i in 0..h {
        for j in 0..w {
            white_sum[i + 1][j + 1] =
                white_sum[i + 1][j] + white_sum[i][j + 1] - white_sum[i][j] + white[i][j];
        }
    }
    let white_sum = white_sum;

    // 2014 年の問題で計算回数高々 (10^2)^4 回を出す？出した
    let mut ans = 0;
    for si in 0..h {
        for sj in 0..w {
            for gi in si..h {
                for gj in sj..w {
                    let cur_blk =
                        black_sum[gi + 1][gj + 1] - black_sum[si][gj + 1] - black_sum[gi + 1][sj]
                            + black_sum[si][sj];
                    let cur_whi =
                        white_sum[gi + 1][gj + 1] - white_sum[si][gj + 1] - white_sum[gi + 1][sj]
                            + white_sum[si][sj];
                    if cur_blk == cur_whi {
                        ans = ans.max((gi - si + 1) * (gj - sj + 1));
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
