// :fu: 21-03 数問
// N <= 10^5 だから N x N の配列はメモリ不足 (RE)

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut rows = (0..n).collect::<Vec<usize>>(); // 行/横
    let mut cols = (0..n).collect::<Vec<usize>>(); // 列/縦
    let mut transposed = false;
    for _ in 0..q {
        input! {
            q0: usize,
        }
        match q0 {
            1 => {
                // 行交換
                input! {
                    a: usize,
                    b: usize,
                }
                let a = a - 1;
                let b = b - 1;
                if transposed {
                    cols.swap(a, b);
                } else {
                    rows.swap(a, b);
                }
            }
            2 => {
                // 列交換
                input! {
                    a: usize,
                    b: usize,
                }
                let a = a - 1;
                let b = b - 1;
                if transposed {
                    rows.swap(a, b);
                } else {
                    cols.swap(a, b);
                }
            }
            3 => {
                transposed = !transposed;
            }
            4 => {
                input! {
                    a: usize,
                    b: usize,
                }
                let a = a - 1;
                let b = b - 1;
                println!(
                    "{}",
                    if transposed {
                        n * rows[b] + cols[a]
                    } else {
                        n * rows[a] + cols[b]
                    }
                );
            }
            _ => unreachable!(),
        }
    }
}
