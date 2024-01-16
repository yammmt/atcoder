use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        x: i64,
        k: i64,
        d: i64,
    }
    let x = x.abs();

    if d <= x / k {
        // 一方向に移動し続ける
        println!("{}", x - k * d);
    } else {
        // 一度に動いた後に 0 付近で反復する
        let k_posi_min = x / d;
        let x_posi_min = x - k_posi_min * d;
        if (k - k_posi_min) % 2 == 0 {
            println!("{x_posi_min}");
        } else {
            println!("{}", (x_posi_min - d).abs());
        }
    }
}
