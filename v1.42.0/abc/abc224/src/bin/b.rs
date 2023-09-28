use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        ahw: [[usize; w]; h],
    }

    for i1 in 0..h {
        for i2 in i1 + 1..h {
            for j1 in 0..w {
                for j2 in j1 + 1..w {
                    if ahw[i1][j1] + ahw[i2][j2] > ahw[i2][j1] + ahw[i1][j2] {
                        println!("No");
                        return;
                    }
                }
            }
        }
    }

    println!("Yes");
}
