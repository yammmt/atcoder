use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }

    // 制約が緩いので愚直に実装する

    // 自身に対しては N 出力でよい
    let mut follows = vec![vec![false; n]; n];

    for _ in 0..q {
        input! {
            qq: usize,
        }
        match qq {
            1 => {
                input! {
                    a: Usize1,
                    b: Usize1,
                }
                follows[a][b] = true;
            }
            2 => {
                input! {
                    a: Usize1,
                }
                for i in 0..n {
                    if follows[i][a] {
                        follows[a][i] = true;
                    }
                }
            }
            3 => {
                input! {
                    a: Usize1,
                }
                let mut to_follow = vec![];
                for i in 0..n {
                    if !follows[a][i] {
                        continue;
                    }

                    for j in 0..n {
                        if follows[i][j] && j != a {
                            to_follow.push(j);
                        }
                    }
                }
                while let Some(i) = to_follow.pop() {
                    follows[a][i] = true;
                }
            }
            _ => unreachable!(),
        }
    }

    for i in 0..n {
        for j in 0..n {
            print!("{}", if follows[i][j] { "Y" } else { "N" });
        }
        println!();
    }
}
