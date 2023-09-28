// 入力の時点でだるい

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut bnm = vec![vec![]; n];
    for i in 0..n {
        input! {
            c: Chars,
        }
        for j in 0..m {
            bnm[i].push(c[j].to_digit(10).unwrap());
        }
    }

    let mut ans = vec![vec![0; m]; n];
    for i in 0..n - 1 {
        for j in 0..m {
            if bnm[i][j] > 0 {
                ans[i + 1][j] = bnm[i][j];
                if j > 0 {
                    bnm[i + 1][j - 1] -= bnm[i][j];
                }
                if j < m - 1 {
                    bnm[i + 1][j + 1] -= bnm[i][j];
                }
                if i + 2 < n {
                    bnm[i + 2][j] -= bnm[i][j];
                }
                bnm[i][j] = 0;
            }
        }
        // println!("ans: {:?}", ans);
        // println!("bnm: {:?}", bnm);
    }

    for i in 0..n {
        for j in 0..m {
            print!("{}", ans[i][j]);
        }
        println!();
    }
}
