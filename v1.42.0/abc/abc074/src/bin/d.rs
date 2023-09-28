// :fu: 復元で摘み
// http://takaxyz.com/2018/11/20/atcoder-bc074-d-restoring-road-network/

use proconio::input;

fn main() {
    input! {
        n: usize,
        ann: [[usize; n]; n],
    }

    let mut ans = vec![vec![std::usize::MAX / 3; n]; n];
    for i in 0..n {
        ans[i][i] = 0;
    }
    let mut is_needed = vec![vec![true; n]; n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if i != k && k != j && ann[i][k] + ann[k][j] == ann[i][j] {
                    // 迂回しても距離が変わらない
                    is_needed[i][j] = false;
                }
                ans[i][j] = ans[i][j].min(ann[i][j].min(ann[i][k] + ann[k][j]));
            }
        }
    }
    // println!("{:?}", ans);
    // println!("{:?}", is_needed);
    if ans != ann {
        println!("-1");
        return;
    }

    let mut path_sum = 0;
    for i in 0..n {
        for j in i + 1..n {
            if is_needed[i][j] {
                path_sum += ann[i][j];
            }
        }
    }
    println!("{}", path_sum);
}
