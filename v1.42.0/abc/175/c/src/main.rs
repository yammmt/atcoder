use proconio::input;

fn main() {
    input! {
        mut x: i64,
        k: i64,
        d: i64,
    }

    // 動けるだけ原点に動いて端数を調整
    x = x.abs();
    let go_straight_num = x / d;
    let x_plus_min = x - d * go_straight_num;
    println!(
        "{}",
        if go_straight_num > k {
            x - k * d
        } else if (go_straight_num - k) % 2 == 0 {
            x_plus_min
        } else {
            (x_plus_min - d).abs()
        }
    );
}
