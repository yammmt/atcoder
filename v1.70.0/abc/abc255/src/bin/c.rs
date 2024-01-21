use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        mut x: i64,
        mut a: i64,
        mut d: i64,
        n: i64,
    }

    // 公差非負, 初項 0 に置き換える
    if d < 0 {
        a = a + d * (n - 1);
        d = d.abs();
    }
    x -= a;
    a -= a;

    let a_last = a + (n - 1) * d;
    println!(
        "{}",
        if x <= 0 {
            a - x
        } else if x >= a_last {
            x - a_last
        } else {
            ((x - a) % d).min(d - (x - a) % d)
        }
    );
}
