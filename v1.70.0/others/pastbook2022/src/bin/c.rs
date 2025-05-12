use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

fn dfs(v_cur: usize, v_parent: usize, edges: &Vec<Vec<usize>>, ans: &mut [usize]) {
    ans[v_cur] = 1;
    for &v_next in &edges[v_cur] {
        if v_next == v_parent {
            continue;
        }

        dfs(v_next, v_cur, edges, ans);
        ans[v_cur] += ans[v_next];
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        abn: [(Usize1, Usize1); n - 1],
    }

    let mut edges = vec![vec![]; n];
    for (a, b) in abn {
        edges[a].push(b);
        edges[b].push(a);
    }

    let mut ans = vec![0; n];
    dfs(0, 0, &edges, &mut ans);

    for a in ans {
        println!("{a}");
    }
}
