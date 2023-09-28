use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        ahw: [[usize; w]; h],
    }

    let mut bwh = vec![vec![0; h]; w];
    for i in 0..h {
        for j in 0..w {
            bwh[j][i] = ahw[i][j];
        }
    }

    for i in 0..w {
        for j in 0..h {
            print!("{}", bwh[i][j]);
            if j != h - 1 {
                print!(" ");
            }
        }
        println!();
    }
}
