use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
    }
    let mut c = vec![];
    let mut p = vec![];
    for _ in 0..n {
        input! {
            cc: usize,
            pc: [usize; cc],
        }
        c.push(cc);
        p.push(pc);
    }

    // トポロジカルソート
    let mut ans = vec![];
    let mut stack = VecDeque::new();
    // 出現回数のカウント: 以下のグラフで正解するため
    // 1 -> 2, 1 -> 3, 2 -> 3
    // このグラフでは正しい順は 2, 3, 1 だが, 単純な到達判定では 3, 2, 1 とも成り得て WA
    // もう少し具体的には, 頂点 2 を確認中に 3 はすでに Stack に入っているのでスルー, とすると WA
    let mut visited_num = vec![0; n];
    stack.push_back(0);
    while let Some(cur) = stack.pop_back() {
        visited_num[cur] += 1;
        if visited_num[cur] == 2 {
            ans.push(cur + 1);
        }
        if visited_num[cur] >= 2 {
            continue;
        }

        // 再帰で書くなら自身の `push_back()` は不要で,
        // 頂点代入時に再帰で頂点代入完了後に ans に入れる
        stack.push_back(cur);
        for v in &p[cur] {
            if visited_num[v - 1] != 2 {
                stack.push_back(v - 1);
            }
        }
    }

    // 0 が末尾に入るので取り除く
    for (i, a) in ans.iter().take(ans.len() - 1).enumerate() {
        print!("{a}");
        if i == ans.len() - 2 {
            println!();
        } else {
            print!(" ");
        }
    }
}
