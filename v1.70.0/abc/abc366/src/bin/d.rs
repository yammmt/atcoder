use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        annn: [[[usize; n]; n]; n],
        q: usize,
        lrxyzq: [(Usize1, Usize1, Usize1, Usize1, Usize1, Usize1); q],
    }

    // 題意がつらい...
    // x を無視して, 二次元累積和をする. 左上から自身までの和を全マス分前計算する.
    // クエリの度に各 x に対してこれを足して答えとする.
    // 前計算部を除いた計算量が最大 2x10^7 くらいになるが, 実行時間制限 3 s だし

    // sum2d[x][y][z]: [x][0][0] から [x][y][z] までの累積和, 区間は閉区間
    let mut sum2d = vec![vec![vec![0; n]; n]; n];

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                let mut cur = annn[i][j][k];
                if j > 0 {
                    cur += sum2d[i][j - 1][k];
                }
                if k > 0 {
                    cur += sum2d[i][j][k - 1];
                }
                if j > 0 && k > 0 {
                    cur -= sum2d[i][j - 1][k - 1];
                }
                sum2d[i][j][k] = cur;
            }
        }
    }

    for (lx, rx, ly, ry, lz, rz) in lrxyzq {
        let mut ans = 0;
        for i in lx..=rx {
            ans += sum2d[i][ry][rz];
            if ly > 0 && lz > 0 {
                ans += sum2d[i][ly - 1][lz - 1];
            }
            if ly > 0 {
                ans -= sum2d[i][ly - 1][rz];
            }
            if lz > 0 {
                ans -= sum2d[i][ry][lz - 1];
            }
        }

        println!("{ans}");
    }
}
