// :fu: 21-03 数問 (幾何) 四則演算を書き間違えない

use proconio::input;

fn main() {
    input! {
        n: usize,
        xy0: (f64, f64),
        xyhalf: (f64, f64),
    }

    let xymid = ((xy0.0 + xyhalf.0) / 2.0, (xy0.1 + xyhalf.1) / 2.0);
    let xyvec = (xy0.0 - xymid.0, xy0.1 - xymid.1);
    let deg = 180.0 / (n as f64 / 2.0);
    let rad = std::f64::consts::PI * deg / 180.0;
    println!(
        "{} {}",
        xyvec.0 * rad.cos() - xyvec.1 * rad.sin() + xymid.0,
        xyvec.0 * rad.sin() + xyvec.1 * rad.cos() + xymid.1
    );
}
