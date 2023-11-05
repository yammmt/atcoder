// ごっちゃになる

use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        ftxq: [(usize, usize, usize); q],
    }

    // 線形リスト
    let mut table_top = Vec::with_capacity(n);
    for i in 0..n {
        table_top.push(Some(i));
    }
    let mut lower_of_me = vec![None; n];

    for ftx in ftxq {
        // f: from, t: to
        let f = ftx.0 - 1;
        let t = ftx.1 - 1;
        let x = ftx.2 - 1;

        // 移動元の最上位が変わる
        let top_of_x = table_top[f];
        table_top[f] = lower_of_me[x];

        // 移動先の下が変わる
        lower_of_me[x] = table_top[t];

        // 移動先の最上位が変わる
        table_top[t] = top_of_x;
    }

    let mut ans = vec![0; n];
    for i in 0..n {
        let mut cur = table_top[i];
        while cur.is_some() {
            let cc = cur.unwrap();
            ans[cc] = i + 1;
            cur = lower_of_me[cc];
        }
    }

    for a in ans {
        println!("{a}");
    }
}
