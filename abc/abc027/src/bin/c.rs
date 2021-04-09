// :fu: 21-04 ゲーム
// https://qiita.com/hamamu/items/55433210be3c47a4dd72

use proconio::input;

fn main() {
    input! {
        mut n: u64,
    }

    let mut tkhs_win = true;
    let mut lr = (n + 1, 2 * n + 1);
    while lr.0 != 1 {
        lr = if tkhs_win {
            ((lr.0 + 1) / 2, lr.0 - 1)
        } else {
            (lr.0 / 2, lr.0 - 1)
        };
        tkhs_win = !tkhs_win;
    }

    println!(
        "{}",
        if tkhs_win {
            "Takahashi"
        } else {
            "Aoki"
        }
    );
}
