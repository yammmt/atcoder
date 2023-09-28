use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        uvm: [(usize, usize); m],
        mut cn: [usize; n],
    }

    let mut edges = vec![vec![]; n];
    for uv in &uvm {
        edges[uv.0 - 1].push(uv.1 - 1);
        edges[uv.1 - 1].push(uv.0 - 1);
    }

    for _ in 0..q {
        input! {
            s0: usize,
        }
        if s0 == 1 {
            input! {
                x: usize,
            }
            println!("{}", cn[x - 1]);
            for v in &edges[x - 1] {
                cn[*v] = cn[x - 1];
            }
        } else {
            input! {
                x: usize,
                y: usize,
            }
            println!("{}", cn[x - 1]);
            cn[x - 1] = y;
        }
    }
}
