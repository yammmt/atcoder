// diff の割には実装重い気がするのだけど
// => 私の実装が賢くない
// thanks: https://atcoder.jp/contests/abc239/submissions/46786469

use proconio::input;
use proconio::marker::Usize1;

fn dfs(
    x: &Vec<usize>,
    edges: &Vec<Vec<usize>>,
    i: usize,
    parent: usize,
    ans: &mut Vec<Vec<usize>>,
) {
    let mut ans_cur = vec![x[i]];

    for &v in &edges[i] {
        // 入力は木だから親に戻ることだけ防げばよい
        if v == parent {
            continue;
        }

        dfs(x, edges, v, i, ans);
        for ans_v in &ans[v] {
            ans_cur.push(*ans_v);
        }
    }

    ans_cur.sort_unstable();
    ans_cur.reverse();
    ans_cur.truncate(20);
    ans[i] = ans_cur;
}

fn main() {
    input! {
        n: usize,
        q: usize,
        xn: [usize; n],
        abn1: [(Usize1, Usize1); n - 1],
        vkq: [(Usize1, Usize1); q],
    }

    let mut edges = vec![vec![]; n];
    for (a, b) in abn1 {
        edges[a].push(b);
        edges[b].push(a);
    }

    let mut ans = vec![vec![]; n];
    dfs(&xn, &edges, 0, 0, &mut ans);

    for (v, k) in vkq {
        println!("{}", ans[v][k]);
    }
}
