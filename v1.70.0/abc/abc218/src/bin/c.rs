use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;

const DUMMY: isize = isize::MAX / 2;

fn is_yes(vgrid: &[Vec<(isize, isize)>], t: usize) -> bool {
    let mut g0_turned = vec![];
    let mut x_min = DUMMY;
    let mut y_min = DUMMY;
    for &p in &vgrid[0] {
        let mut x = p.0;
        let mut y = p.1;
        // 回転
        for _ in 0..t {
            let tmp = x;
            x = y;
            y = -tmp;
        }
        x_min = x_min.min(x);
        y_min = y_min.min(y);
        g0_turned.push((x, y));
    }

    let mut g0_all = HashSet::new();
    // 左上に詰める (平行移動)
    for p in g0_turned {
        g0_all.insert((p.0 - x_min, p.1 - y_min));
    }

    for p_g1 in &vgrid[1] {
        if !g0_all.contains(p_g1) {
            return false;
        }
    }

    true
}

#[fastout]
fn main() {
    input! {
        n: usize,
        snn: [Chars; n],
        tnn: [Chars; n],
    }

    let mut vst = vec![];
    for grid in [snn, tnn] {
        let mut cur_raw = vec![];
        let mut x_min = DUMMY;
        let mut y_min = DUMMY;
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == '#' {
                    x_min = x_min.min(i as isize);
                    y_min = y_min.min(j as isize);
                    cur_raw.push((i as isize, j as isize));
                }
            }
        }
        let mut cur = vec![];
        for cr in cur_raw {
            cur.push((cr.0 - x_min, cr.1 - y_min));
        }
        vst.push(cur);
    }
    if vst[0].len() != vst[1].len() {
        println!("No");
        return;
    }

    let turn_num = [0, 1, 2, 3];

    for t in turn_num {
        if is_yes(&vst, t) {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
