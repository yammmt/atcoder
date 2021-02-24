// 26min (1TLE: 15.5min)
// TLE: 愚直に同じ計算を繰り返してしまった

use proconio::input;

fn main() {
    input! {
        n: usize,
        c: usize,
        dcc: [[usize; c]; c],
        cnn: [[usize; n]; n],
    }

    // costs[i][j]: 色 i を j ([0, 2]) に塗る
    let mut costs = vec![vec![0; 3]; c];
    for i in 0..c {
        for j in 0..3 {
            let mut cur = 0;
            for ii in 0..n {
                for jj in 0..n {
                    if (ii + jj) % 3 != j {
                        continue;
                    }

                    cur += dcc[cnn[ii][jj] - 1][i];
                }
            }
            costs[i][j] = cur;
        }
    }
    // println!("{:?}", costs);

    let mut ans = std::usize::MAX / 2;
    for i in 0..c {
        for j in 0.. c {
            for k in 0..c {
                if i == j || j == k || k == i {
                    continue;
                }

                ans = ans.min(costs[i][0] + costs[j][1] + costs[k][2]);
            }
        }
    }

    println!("{}", ans);
}
