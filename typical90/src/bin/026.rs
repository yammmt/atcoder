use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        abn: [(usize, usize); n - 1],
    }

    let mut edges = vec![vec![]; n];
    for ab in &abn {
        edges[ab.0 - 1].push(ab.1 - 1);
        edges[ab.1 - 1].push(ab.0 - 1);
    }

    let mut selected = vec![false; n];
    let mut visited = vec![false; n];
    let mut vdq = VecDeque::new();
    vdq.push_back((0, true));
    while let Some(cur) = vdq.pop_back() {
        selected[cur.0] = cur.1;
        visited[cur.0] = true;
        for e in &edges[cur.0] {
            if visited[*e] {
                continue;
            }

            vdq.push_back((*e, !cur.1));
        }
    }

    let selected_printed = selected.iter().filter(|&s| *s).count() >= n / 2;

    let mut cur_n = 0;
    for (i, s) in selected.iter().enumerate() {
        if (selected_printed && !s) || (!selected_printed && *s) {
            continue;
        }

        print!("{}", i + 1);
        cur_n += 1;
        if cur_n == n / 2 {
            println!();
            return;
        } else {
            print!(" ");
        }
    }
}
