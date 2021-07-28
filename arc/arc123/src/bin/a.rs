// :fu: :fu: :fu: 21-07 案の定 体感緑後半

use proconio::input;

fn main() {
    input! {
        a3: [i64; 3],
    }

    println!(
        "{}",
        if 2 * a3[1] - a3[0] - a3[2] >= 0 {
            2 * a3[1] - a3[0] - a3[2]
        } else if (a3[0] + a3[2] - 2 * a3[1]) % 2 == 0 {
            (a3[0] + a3[2] - 2 * a3[1]) / 2
        } else {
            (a3[0] + a3[2] - 2 * a3[1] + 1) / 2 + 1
        }
    );
}
