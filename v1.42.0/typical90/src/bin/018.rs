// :fu: :fu: :fu: 21-04 数問 (幾何) 投票でも茶の範囲だが母集団数学できすぎないか

use proconio::input;

fn main() {
    input! {
        t: f64,
        l: f64,
        x: f64,
        y: f64,
        q: usize,
        eq: [f64; q],
    }

    for e in &eq {
        let rad = (360.0 * e / t) * std::f64::consts::PI / 180.0;
        let kanransha = (0.0, (-l / 2.0) * rad.sin(), l / 2.0 - (l / 2.0) * rad.cos());
        let teihen = (x * x + (y - kanransha.1) * (y - kanransha.1)).sqrt();
        let takasa = kanransha.2;
        println!(
            "{}",
            (takasa / teihen).atan() * 180.0 / std::f64::consts::PI
        );
    }
}
