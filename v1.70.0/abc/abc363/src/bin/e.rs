use proconio::fastout;
use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        y: usize,
        ahw: [[usize; w]; h],
    }
    let dir = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    // 雑に Dijkstra 投げると時間が心許ない
    // heap に全ツッコミ -> 1191 ms
    // 沈没済みを省く -> 548 ms
    // 挿入済みを省く -> 204 ms
    // Editorial の "海に接した瞬間に何年目に沈むかが直ちにわかる" で計算量削減できる

    let mut is_alive = vec![vec![true; w]; h];
    let mut is_pushed = vec![vec![false; w]; h];
    let mut alive_num = h * w;
    // (高さ, (座標))
    let mut heap = BinaryHeap::new();
    for i in 0..h {
        heap.push(Reverse((ahw[i][0], (i, 0))));
        heap.push(Reverse((ahw[i][w - 1], (i, w - 1))));
        is_pushed[i][0] = true;
        is_pushed[i][w - 1] = true;
    }
    for i in 0..w {
        heap.push(Reverse((ahw[0][i], (0, i))));
        heap.push(Reverse((ahw[h - 1][i], (h - 1, i))));
        is_pushed[0][i] = true;
        is_pushed[h - 1][i] = true;
    }

    for turn in 1..=y {
        while let Some(Reverse((a_cur, (i_cur, j_cur)))) = heap.pop() {
            if a_cur > turn {
                heap.push(Reverse((a_cur, (i_cur, j_cur))));
                break;
            }

            if !is_alive[i_cur][j_cur] {
                continue;
            }

            is_alive[i_cur][j_cur] = false;
            alive_num -= 1;

            for &d in &dir {
                let i_nxt = i_cur.wrapping_add_signed(d.0);
                let j_nxt = j_cur.wrapping_add_signed(d.1);
                if i_nxt >= h || j_nxt >= w || is_pushed[i_nxt][j_nxt] {
                    continue;
                }

                heap.push(Reverse((ahw[i_nxt][j_nxt], (i_nxt, j_nxt))));
                is_pushed[i_nxt][j_nxt] = true;
            }
        }

        println!("{alive_num}");
    }
}
