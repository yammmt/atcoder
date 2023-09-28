// :fu: 数問 (幾何) でインデックスが嫌
// 32min (理想解というよりは言語の速度で押した)
// https://qiita.com/drken/items/56a6b68edef8fc605821

use proconio::input;

fn main() {
    input! {
        n: usize,
        dnn: [[u64; n]; n],
        q: usize,
        pq: [usize; q],
    }

    let mut areasum = vec![vec![0; n + 1]; n + 1];
    for i in 0..n {
        for j in 0..n {
            areasum[i + 1][j + 1] = areasum[i][j + 1] + areasum[i + 1][j] - areasum[i][j] + dnn[i][j];
        }
    }

    let mut ans = vec![0; n * n + 1];
    for x1 in 0..n {
        for x2 in x1 + 1..n + 1 {
            for y1 in 0..n {
                for y2 in y1 + 1..n + 1 {
                    let area = (x2 - x1) * (y2 - y1);
                    let sum = areasum[x2][y2] + areasum[x1][y1] - areasum[x1][y2] - areasum[x2][y1];
                    ans[area] = ans[area].max(sum);
                }
            }
        }
    }
    for i in 1..n * n + 1 {
        ans[i] = ans[i].max(ans[i - 1]);
    }

    for p in &pq {
        println!("{}", ans[*p]);
    }
}
