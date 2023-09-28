// マス目が厄介

use petgraph::unionfind::UnionFind;
use proconio::input;

fn hw_to_1d(cur: (usize, usize), _h: usize, w: usize) -> usize {
    w * cur.0 + cur.1
}

fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
    }
    let dir = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    let mut is_red = vec![vec![false; w]; h];
    let mut uf = UnionFind::new(h * w);
    for _ in 0..q {
        input! {
            n: usize,
        }
        if n == 1 {
            input! {
                r: usize,
                c: usize,
            }
            let rr = r - 1;
            let cc = c - 1;
            is_red[rr][cc] = true;
            for d in &dir {
                let h_i = rr as isize + d.0;
                let w_i = cc as isize + d.1;
                if h_i < 0 || w_i < 0 || h_i >= h as isize || w_i >= w as isize {
                    continue;
                }

                let h_u = h_i as usize;
                let w_u = w_i as usize;
                if is_red[h_u][w_u] {
                    uf.union(hw_to_1d((rr, cc), h, w), hw_to_1d((h_u, w_u), h, w));
                }
            }
        } else {
            input! {
                ra: (usize, usize),
                rb: (usize, usize),
            }
            let raa = (ra.0 - 1, ra.1 - 1);
            let rbb = (rb.0 - 1, rb.1 - 1);
            println!(
                "{}",
                if is_red[raa.0][raa.1]
                    && is_red[rbb.0][rbb.1]
                    && uf.find(hw_to_1d(raa, h, w)) == uf.find(hw_to_1d(rbb, h, w))
                {
                    "Yes"
                } else {
                    "No"
                }
            );
        }
    }
}
