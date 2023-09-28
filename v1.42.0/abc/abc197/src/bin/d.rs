// :fu: 21-11 数問 (幾何)

use proconio::input;

fn main() {
    input! {
        n: f64,
        xy0: (f64, f64),
        xyh: (f64, f64),
    }

    // 図形原点をこことする
    let offset = ((xy0.0 + xyh.0) / 2.0, (xy0.1 + xyh.1) / 2.0);
    let xy00 = (xy0.0 - offset.0, xy0.1 - offset.1);
    let deg = 360.0 / n;
    let rad = deg * std::f64::consts::PI / 180.0;

    println!(
        "{} {}",
        xy00.0 * rad.cos() - xy00.1 * rad.sin() + offset.0,
        xy00.0 * rad.sin() + xy00.1 * rad.cos() + offset.1
    );
}
