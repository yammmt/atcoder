// :fu: :fu: 21-07 もう少しわかって良い難易度帯

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        uvm: [(usize, usize); m],
    }
    let d = 998_244_353;
    // 道が "使えなくなった"
    // M <= 5,000 の時点でこれを活用しないと TLE する
    // 削除を考えた余事象？
    let mut deleted_edges = vec![HashSet::new(); n];
    for uv in &uvm {
        deleted_edges[uv.0 - 1].insert(uv.1 - 1);
        deleted_edges[uv.1 - 1].insert(uv.0 - 1);
    }

    let mut dp = vec![0; n];
    dp[0] = 1;
    for _ in 0..k {
        let mut next = vec![dp.iter().sum::<i64>(); n];
        for i in 0..n {
            next[i] = ((next[i] + d) - dp[i]) % d;
            for de in &deleted_edges[i] {
                next[i] = ((next[i] + d) - dp[*de]) % d;
            }
        }
        // println!("{:?}", next);
        dp = next;
    }

    println!("{}", dp[0]);
}
