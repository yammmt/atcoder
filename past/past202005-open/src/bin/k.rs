// 線形リストは珍しい気がする
// 個数指定がなければ UnionFind で良いのだけれど

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        // 机 f にあるコンテナ x 以上を机 t に移動する
        ftxq: [(usize, usize, usize); q],
    }

    // テーブルから見て一番上にあるコンテナ
    let mut table_top = vec![];
    for i in 0..n + 1 {
        table_top.push(Some(i));
    }
    // コンテナから見て自分の直下にあるコンテナ
    let mut container_prev = vec![None; n + 1];
    for ftx in &ftxq {
        // 更新順が事故を招く
        // - 動かしたコンテナの下を移動先机の一番上にする
        // - 移動元机の一番上を動かしたコンテナの直下にする
        // - 移動先机の一番上を移動元机の一番上にする

        let moved_container_prev = container_prev[ftx.2];
        let from_table_top = table_top[ftx.0];
        let to_table_top = table_top[ftx.1];

        container_prev[ftx.2] = to_table_top;
        table_top[ftx.0] = moved_container_prev;
        table_top[ftx.1] = from_table_top;
    }
    // println!("t: {:?}", table_top);
    // println!("c: {:?}", container_prev);

    let mut ans = vec![0; n + 1];
    for i in 1..n + 1 {
        if table_top[i] == None {
            continue;
        }

        let mut curr = table_top[i];
        while let Some(cur) = curr {
            // println!("{:?} ", curr);
            ans[cur] = i;
            curr = container_prev[cur];
            // println!("-> {:?}", curr);
        }
    }

    for a in ans.iter().skip(1) {
        println!("{}", a);
    }
}
