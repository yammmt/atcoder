// WA: kij ループの位置
// テストケース非公開

use permutohedron::heap_recursive;
use proconio::input;

#[allow(clippy::needless_range_loop)]
fn main() {
    input! {
        n: usize,
        m: usize,
        r: usize,
        mut rr: [usize; r],
        abcm: [(usize, usize, usize); m],
    }

    let mut costs = vec![vec![std::usize::MAX >> 1; n]; n];
    // [i][i] を 0 にしてもしなくても同じはずだが
    for i in 0..n {
        costs[i][i] = 0;
    }
    // 制約より i/j は重複しない
    for abc in &abcm {
        costs[abc.0 - 1][abc.1 - 1] = abc.2;
        costs[abc.1 - 1][abc.0 - 1] = abc.2;
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                costs[i][j] = costs[i][j].min(costs[i][k] + costs[k][j]);
            }
        }
    }
    // println!("costs: {:?}", costs);

    let mut ans = std::usize::MAX;
    heap_recursive(&mut rr, |p| {
        // println!("{:?}", p);
        let mut cur = 0;
        for i in 1..p.len() {
            if p[i - 1] == p[i] {
                panic!();
            }
            cur += costs[p[i - 1] - 1][p[i] - 1];
        }
        ans = ans.min(cur);
    });

    println!("{}", ans);
}
