use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        uvm: [(usize, usize); m],
        mut cn: [usize; n],
    }

    let mut edges = vec![vec![]; n];
    for uv in uvm {
        let u = uv.0 - 1;
        let v = uv.1 - 1;
        edges[u].push(v);
        edges[v].push(u);
    }

    for _ in 0..q {
        input! {
            nn: usize,
        }
        match nn {
            1 => {
                input! {
                    x: usize,
                }
                println!("{}", cn[x - 1]);
                for v in &edges[x - 1] {
                    cn[*v] = cn[x - 1];
                }
            }
            2 => {
                input! {
                    x: usize,
                    y: usize,
                }
                println!("{}", cn[x - 1]);
                cn[x - 1] = y;
            }
            _ => unreachable!(),
        }
    }
}
