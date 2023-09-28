// :fu: :fu: 21-02
// 解説放送の方針はわかるが次数 1 (葉) から BFS して小さい数を貪欲は嘘解法では？

use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        abn: [(usize, usize); n - 1],
        mut cn: [i64; n],
    }
    cn.sort();
    cn.reverse();

    let mut edges = vec![vec![]; n];
    for ab in &abn {
        edges[ab.0 - 1].push(ab.1 - 1);
        edges[ab.1 - 1].push(ab.0 - 1);
    }

    let mut ans = vec![0; n];
    let mut st = VecDeque::new();
    st.push_back(0);
    ans[0] = cn[0];
    let mut c_idx = 0;
    while let Some(cur) = st.pop_back() {
        ans[cur] = cn[c_idx];
        c_idx += 1;
        for v in &edges[cur] {
            if ans[*v] != 0 {
                continue;
            }

            st.push_back(*v);
        }
    }

    println!("{}", cn.iter().sum::<i64>() - cn[0]);
    for (i, a) in ans.iter().enumerate() {
        print!("{}", a);
        if i == n - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
}
