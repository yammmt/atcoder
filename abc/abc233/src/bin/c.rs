// C でグラフが出るのが日常化してきた

use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        x: u128,
    }
    let mut ln = vec![];
    let mut anl = vec![];
    for _ in 0..n {
        input! {
            l: usize,
            al: [u128; l],
        }
        ln.push(l);
        anl.push(al);
    }

    // 個数の総積が 10^5 以下であり全探索できる
    // 各行の長さが異なるので bit 全探索は無理
    // 入力に 0 なし
    let mut ans = 0;
    // (今の総和, 次の idx)
    let mut vdq = VecDeque::new();
    vdq.push_back((1, 0));
    while let Some(cur) = vdq.pop_front() {
        for a in &anl[cur.1] {
            let next_mul = cur.0 * *a;
            if cur.1 == n - 1 && next_mul == x {
                ans += 1;
            }

            if next_mul <= x && cur.1 < n - 1 {
                vdq.push_back((next_mul, cur.1 + 1));
            }
        }
    }

    println!("{}", ans);
}
