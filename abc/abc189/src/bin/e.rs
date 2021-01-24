// :fu:
// アフィン変換知らないとググっても式に辿り着けず 導出がわからない
// 高校数学でアフィン変換を扱う時代？ 2x2 行列と平行移動量？
// 実装も重くついでに行列もこのレート帯では珍しく本当に水色？

use proconio::input;

fn main() {
    input! {
        n: usize,
        xyn: [(i64, i64); n],
        m: usize,
    }

    let mut vmv = Vec::with_capacity(m + 1);
    vmv.push(vec![[1i64, 0i64, 0i64], [0, 1, 0], [0, 0, 1]]);
    for i in 0..m {
        input! {
            op: usize,
        }

        let cur = match op {
            // clockwise
            1 => vec![[0, 1, 0], [-1, 0, 0], [0, 0, 1]],
            // counter clockwise
            2 => vec![[0, -1, 0], [1, 0, 0], [0, 0, 1]],
            o => {
                input! {
                    p: i64,
                }
                if o == 3 {
                    // y 軸対称 (x = p)
                    vec![[-1, 0, 2 * p], [0, 1, 0], [0, 0, 1]]
                } else {
                    // x 軸対称 (y = p)
                    vec![[1, 0, 0], [0, -1, 2 * p], [0, 0, 1]]
                }
            }
        };

        let mut v = vec![[0, 0, 0]; 3];
        // println!("{:?}", v);
        // println!("{:?}", cur);
        // println!("{:?}", vmv);
        // println!();
        for ii in 0..3 {
            for jj in 0..3 {
                for kk in 0..3 {
                    v[ii][jj] += cur[ii][kk] * vmv[i][kk][jj];
                }
            }
        }
        vmv.push(v);
    }
    // println!("{:?}", vmv);

    input! {
        q: usize,
        abq: [(usize, usize); q],
    }
    // ab.0 回目の操作後の ab.1 の座標
    for ab in &abq {
        // 非正方行列の積をうまく書くのは諦める
        let x = xyn[ab.1 - 1].0;
        let y = xyn[ab.1 - 1].1;
        println!(
            "{} {}",
            vmv[ab.0][0][0] * x + vmv[ab.0][0][1] * y + vmv[ab.0][0][2],
            vmv[ab.0][1][0] * x + vmv[ab.0][1][1] * y + vmv[ab.0][1][2]
        );
    }
}
