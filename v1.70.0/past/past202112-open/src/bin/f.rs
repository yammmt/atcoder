use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;
use proconio::marker::Usize1;
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        a: Usize1,
        b: Usize1,
        s33: [Chars; 3],
    }

    let mut visited = vec![vec![false; 9]; 9];
    let mut que = VecDeque::new();
    que.push_back((a, b));
    while let Some(cur) = que.pop_back() {
        if visited[cur.0][cur.1] {
            continue;
        }

        visited[cur.0][cur.1] = true;
        // マスの移動がすんなり入ってこない
        for i in 0..3 {
            let a_nxt = cur.0.wrapping_add_signed(i - 1);
            if a_nxt >= 9 {
                continue;
            }

            for j in 0..3 {
                let b_nxt = cur.1.wrapping_add_signed(j - 1);
                if b_nxt >= 9 || visited[a_nxt][b_nxt] || s33[i as usize][j as usize] == '.' {
                    continue;
                }

                que.push_back((a_nxt, b_nxt));
            }
        }
    }

    let mut ans = 0;
    for i in 0..9 {
        for j in 0..9 {
            if visited[i][j] {
                ans += 1;
            }
        }
    }

    println!("{ans}");
}
