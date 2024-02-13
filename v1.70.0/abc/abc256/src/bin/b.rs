use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        an: [usize; n],
    }

    let mut p = 0;
    let mut grid = vec![0; 4];
    for a in an {
        grid[0] += 1;
        let mut grid_nxt = vec![0; 4];
        for i in 0..4 {
            let i_nxt = i + a;
            if i_nxt > 3 {
                p += grid[i];
            } else {
                grid_nxt[i_nxt] = grid[i];
            }
        }
        grid = grid_nxt;
    }

    println!("{p}");
}
