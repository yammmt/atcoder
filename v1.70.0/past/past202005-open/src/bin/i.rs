use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }

    // 行/列がどこにいったかを管理する
    // 転置が入ったら行と列を入れ替える
    let mut row = (0..=n).collect::<Vec<usize>>();
    let mut col = (0..=n).collect::<Vec<usize>>();
    let mut transposed = false;
    for _ in 0..q {
        input! {
            q0: usize,
        }
        match q0 {
            1 => {
                input! {
                    a: usize,
                    b: usize,
                }
                if transposed {
                    col.swap(a, b);
                } else {
                    row.swap(a, b);
                }
            }
            2 => {
                input! {
                    a: usize,
                    b: usize,
                }
                if transposed {
                    row.swap(a, b);
                } else {
                    col.swap(a, b);
                }
            }
            3 => transposed = !transposed,
            4 => {
                input! {
                    a: usize,
                    b: usize,
                }

                // 1-origin としたので問題文の式そのまま
                println!(
                    "{}",
                    if transposed {
                        n * (row[b] - 1) + (col[a] - 1)
                    } else {
                        n * (row[a] - 1) + (col[b] - 1)
                    }
                );
            }
            _ => unreachable!(),
        }
    }
}
