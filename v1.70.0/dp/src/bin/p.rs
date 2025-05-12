use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

const MOD: usize = 1_000_000_007;

fn dfs(
    v_cur: usize,
    v_parent: usize,
    edges: &Vec<Vec<usize>>,
    dp_black: &mut [usize],
    dp_white: &mut [usize],
) {
    dp_black[v_cur] = 1;
    dp_white[v_cur] = 1;

    for &v_next in &edges[v_cur] {
        if v_next == v_parent {
            continue;
        }

        dfs(v_next, v_cur, edges, dp_black, dp_white);

        dp_white[v_cur] *= dp_white[v_next] + dp_black[v_next];
        dp_black[v_cur] *= dp_white[v_next];
        dp_white[v_cur] %= MOD;
        dp_black[v_cur] %= MOD;
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        xyn: [(Usize1, Usize1); n - 1],
    }

    let mut edges = vec![vec![]; n];
    for (x, y) in xyn {
        edges[x].push(y);
        edges[y].push(x);
    }
    let edges = edges;

    // 始点逆に取る分で片方見て *2 じゃだめ？
    // => 問題文を読もう, 白黒で処理が違う
    let mut dp_black = vec![0; n];
    let mut dp_white = vec![0; n];
    dfs(0, 0, &edges, &mut dp_black, &mut dp_white);

    println!("{}", (dp_black[0] + dp_white[0]) % MOD);
}
