use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        ahw: [Chars; h],
    }

    let mut vdist = vec![vec![std::usize::MAX; w]; h];
    let mut vdq = VecDeque::new();
    for i in 0..h {
        for j in 0..w {
            if ahw[i][j] == '#' {
                vdq.push_back((i, j, 0));
                vdist[i][j] = 0;
            }
        }
    }

    let dir = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    while let Some(cur) = vdq.pop_front() {
        for d in &dir {
            let next_mass = (cur.0 as isize + d.0, cur.1 as isize + d.1);
            if (next_mass.0 < 0
                || next_mass.0 > (h - 1) as isize
                || next_mass.1 < 0
                || next_mass.1 > (w - 1) as isize)
                || vdist[next_mass.0 as usize][next_mass.1 as usize] != std::usize::MAX
            {
                continue;
            }

            vdq.push_back((next_mass.0 as usize, next_mass.1 as usize, cur.2 + 1));
            vdist[next_mass.0 as usize][next_mass.1 as usize] = cur.2 + 1;
        }
    }
    // println!("{:?}", vdist);

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            ans = ans.max(vdist[i][j]);
        }
    }
    println!("{}", ans);
}
