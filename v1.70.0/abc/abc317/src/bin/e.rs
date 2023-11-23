use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

static DUMMY: usize = usize::MAX / 4;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        ahw: [Chars; h],
    }
    let dir = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    // 通れないマスを確定させる
    // 行/列単位に左右/上下で判定する
    let mut is_safe = vec![vec![true; w]; h];
    for i in 0..h {
        for j in 0..w {
            if ahw[i][j] == 'S' || ahw[i][j] == 'G' || ahw[i][j] == '.' {
                continue;
            }

            is_safe[i][j] = false;
            let cur_d = match ahw[i][j] {
                'v' => (1, 0),
                '^' => (-1, 0),
                '>' => (0, 1),
                '<' => (0, -1),
                _ => (0, 0),
            };
            if cur_d == (0, 0) {
                continue;
            }

            let mut x = i as isize + cur_d.0;
            let mut y = j as isize + cur_d.1;
            loop {
                if x < 0
                    || y < 0
                    || x >= h as isize
                    || y >= w as isize
                    || ahw[x as usize][y as usize] != '.'
                {
                    break;
                }

                is_safe[x as usize][y as usize] = false;
                x += cur_d.0;
                y += cur_d.1;
            }
        }
    }

    let mut start = (0, 0);
    let mut goal = (0, 0);
    for i in 0..h {
        for j in 0..w {
            if ahw[i][j] == 'S' {
                start = (i, j);
            } else if ahw[i][j] == 'G' {
                goal = (i, j);
            }
        }
    }

    let mut que = VecDeque::new();
    let mut costs = vec![vec![DUMMY; w]; h];
    // (cost, (i, j))
    que.push_back((0, start));
    while let Some(cur) = que.pop_front() {
        let c = cur.0;
        let x = cur.1 .0;
        let y = cur.1 .1;
        if costs[x][y] != DUMMY {
            continue;
        }

        costs[x][y] = c;
        for d in &dir {
            // println!("({x}, {y}) += ({}, {})", d.0, d.1);
            let x_nxt = x.wrapping_add_signed(d.0);
            let y_nxt = y.wrapping_add_signed(d.1);
            if x_nxt >= h || y_nxt >= w || costs[x_nxt][y_nxt] != DUMMY || !is_safe[x_nxt][y_nxt] {
                continue;
            }

            que.push_back((c + 1, (x_nxt, y_nxt)));
        }
    }

    println!(
        "{}",
        if costs[goal.0][goal.1] == DUMMY {
            -1
        } else {
            costs[goal.0][goal.1] as isize
        }
    );
}
