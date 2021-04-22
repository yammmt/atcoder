use proconio::input;

#[allow(clippy::needless_range_loop)]
fn main() {
    input! {
        h: usize,
        w: usize,
        ahw: [[i32; w]; h],
    }

    let mut h_sum = vec![];
    for i in 0..h {
        let mut cur = 0;
        for j in 0..w {
            cur += ahw[i][j];
        }
        h_sum.push(cur);
    }

    let mut w_sum = vec![];
    for j in 0..w {
        let mut cur = 0;
        for i in 0..h {
            cur += ahw[i][j];
        }
        w_sum.push(cur);
    }

    for i in 0..h {
        for j in 0..w {
            print!("{}", h_sum[i] + w_sum[j] - ahw[i][j]);
            if j != w - 1 {
                print!(" ");
            }
        }
        println!();
    }
}
