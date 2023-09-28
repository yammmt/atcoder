// :fu: 21-11
// WA: 探索初期値

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        an: [u64; n],
    }

    let mut pass = 0;
    let mut fail = an.iter().sum::<u64>() + 1;
    while fail - pass > 1 {
        // mid 個のプロジェクトを作れるか？
        let mid = (fail + pass) / 2;

        let mut completed_col_num = 0;
        let mut cur_row = 0;
        for a in &an {
            let next_row = cur_row + (*a).min(mid);
            if next_row >= mid {
                completed_col_num += 1;
            }
            cur_row = next_row % mid;
        }

        if completed_col_num >= k {
            pass = mid;
        } else {
            fail = mid;
        }
    }

    println!("{}", pass);
}
