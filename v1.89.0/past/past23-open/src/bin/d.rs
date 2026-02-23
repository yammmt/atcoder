use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        c: [char; 2],
        sh: [Chars; h],
    }

    let dir = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for i in 0..h {
        for j in 0..w {
            if sh[i][j] != c[0] {
                continue;
            }

            for &d in &dir {
                let ii = i.wrapping_add_signed(d.0);
                let jj = j.wrapping_add_signed(d.1);
                if ii >= h || jj >= w {
                    continue;
                }

                if sh[ii][jj] == c[1] {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
}
