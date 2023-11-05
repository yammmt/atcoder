// B?

use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
    }

    let mut is_ac = vec![vec![false; m]; n];
    let mut ac_num = vec![0isize; m];
    for _ in 0..q {
        input! {
            s0: usize,
        }
        match s0 {
            1 => {
                // 変数名衝突
                input! {
                    nn: usize,
                }
                let mut pts = 0;
                for i in 0..m {
                    if is_ac[nn - 1][i] {
                        pts += ((n as isize) - ac_num[i]).max(0);
                    }
                }
                println!("{pts}");
            }
            2 => {
                input! {
                    nn: usize,
                    m: usize,
                }
                is_ac[nn - 1][m - 1] = true;
                ac_num[m - 1] += 1;
            }
            _ => unreachable!(),
        }
    }
}
