use proconio::fastout;
use proconio::input;
use std::collections::HashSet;
use std::collections::VecDeque;

const GRID_MAX_ABS: isize = 201;

#[fastout]
fn main() {
    input! {
        n: usize,
        x: isize,
        y: isize,
        xyn: [(isize, isize); n],
    }
    let dir = [(1, 1), (0, 1), (-1, 1), (1, 0), (-1, 0), (0, -1)];

    let mut visited = HashSet::new();
    let disabled_area: HashSet<(isize, isize)> = HashSet::from_iter(xyn.iter().cloned());
    // ((x, y), cost)
    let mut vdq = VecDeque::from([((0, 0), 0)]);
    while let Some(((x_cur, y_cur), cost_cur)) = vdq.pop_front() {
        if visited.contains(&(x_cur, y_cur)) {
            continue;
        } else if x_cur == x && y_cur == y {
            println!("{cost_cur}");
            return;
        }

        visited.insert((x_cur, y_cur));
        for d in &dir {
            let x_nxt = x_cur + d.0;
            let y_nxt = y_cur + d.1;
            let pt_nxt = (x_nxt, y_nxt);
            if x_nxt.abs() > GRID_MAX_ABS
                || y_nxt.abs() > GRID_MAX_ABS
                || visited.contains(&pt_nxt)
                || disabled_area.contains(&pt_nxt)
            {
                continue;
            }

            vdq.push_back((pt_nxt, cost_cur + 1));
        }
    }

    println!("-1");
}
