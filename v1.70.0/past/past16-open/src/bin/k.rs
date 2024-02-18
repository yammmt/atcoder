use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

const DUMMY: usize = usize::MAX / 4;

#[fastout]
fn main() {
    input! {
        n: usize,
        snn: [Chars; n],
    }

    // editorial の k マス進めるか判定は穴の数を数えるという意

    let mut holes_lr = vec![vec![0; n + 1]; n + 1];
    for i in 0..n {
        for j in 0..n {
            holes_lr[i + 1][j + 1] = if snn[i][j] == 'X' {
                holes_lr[i + 1][j] + 1
            } else {
                holes_lr[i + 1][j]
            };
        }
    }
    let mut holes_ud = vec![vec![0; n + 1]; n + 1];
    for i in 0..n {
        for j in 0..n {
            holes_ud[j + 1][i + 1] = if snn[j][i] == 'X' {
                holes_ud[j][i + 1] + 1
            } else {
                holes_ud[j][i + 1]
            };
        }
    }
    // println!("{:?}", holes_lr);
    // println!("{:?}", holes_ud);

    let mut ij_start = (0, 0);
    'i_loop: for i in 0..n {
        for j in 0..n {
            if snn[i][j] == 'S' {
                ij_start = (i, j);
                break 'i_loop;
            }
        }
    }

    let could_move = |mv_from: (usize, usize), mv_to: (usize, usize)| {
        let mv_lr = mv_from.0 == mv_to.0;
        let mv_ud = mv_from.1 == mv_to.1;
        mv_from.0 < n
            && mv_from.1 < n
            && mv_to.0 < n
            && mv_to.1 < n
            && (!mv_lr || holes_lr[mv_from.0 + 1][mv_from.1] == holes_lr[mv_to.0 + 1][mv_to.1 + 1])
            && (!mv_ud || holes_ud[mv_from.0 + 1][mv_from.1 + 1] == holes_ud[mv_to.0 + 1][mv_to.1 + 1])
            && snn[mv_from.0][mv_from.1] != 'X'
            && snn[mv_to.0][mv_to.1] != 'X'
    };

    for k in 1..n {
        let mut ans = DUMMY;
        let mut que = VecDeque::new();
        que.push_back((ij_start, 0));
        let mut visited = vec![vec![false; n]; n];
        while let Some(((i_cur, j_cur), score_cur)) = que.pop_front() {
            if visited[i_cur][j_cur] {
                continue;
            } else if snn[i_cur][j_cur] == 'G' {
                ans = score_cur;
                break;
            }

            // println!(" {i_cur} {j_cur}");
            visited[i_cur][j_cur] = true;
            let want_to_move = vec![
                (i_cur, j_cur + k),
                (i_cur, j_cur.wrapping_add_signed(k as isize * -1)),
                (i_cur + k, j_cur),
                (i_cur.wrapping_add_signed(k as isize * -1), j_cur),
            ];
            for wm in &want_to_move {
                if could_move((i_cur, j_cur), *wm) {
                    // println!("   -> {:?}", wm);
                    que.push_back((*wm, score_cur + 1));
                }
            }
        }

        if ans == DUMMY {
            println!("-1");
        } else {
            println!("{ans}");
        }
        // println!("{:?}", visited);
        // println!("k: {k}");
        // for i in 0..n {
        //     println!("{:?}", visited[i]);
        // }
        // return;
    }
}
