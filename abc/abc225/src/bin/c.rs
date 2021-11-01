// :fu: 21-11

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        bnm: [[usize; m]; n],
    }

    for i in 0..n {
        for j in 0..m {
            if bnm[i][j] == 0 {
                println!("No");
                return;
            }
        }
    }

    // 左上のマスを固定して全走査
    let start_from = (bnm[0][0] / 7, (bnm[0][0] - 1) % 7 + 1);
    // println!("{:?}", start_from);
    if start_from.1 + m - 1 > 7 {
        println!("No");
        return;
    }

    // 横方向の差分
    for i in 0..n {
        for j in 0..m - 1 {
            if bnm[i][j] + 1 != bnm[i][j + 1] {
                println!("No");
                return;
            }
        }
    }
    // 縦方向
    for j in 0..m {
        for i in 0..n - 1 {
            if bnm[i][j] + 7 != bnm[i + 1][j] {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}
