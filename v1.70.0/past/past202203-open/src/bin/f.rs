use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        ahw: [[Usize1; w]; h],
        cn: [usize; n],
    }
    let dir = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for i in 0..h {
        for j in 0..w {
            for d in &dir {
                let ii = i.wrapping_add_signed(d.0);
                let jj = j.wrapping_add_signed(d.1);
                if ii >= h || jj >= w {
                    continue;
                }

                if ahw[i][j] != ahw[ii][jj] && cn[ahw[i][j]] == cn[ahw[ii][jj]] {
                    println!("No");
                    return;
                }
            }
        }
    }

    println!("Yes");
}
