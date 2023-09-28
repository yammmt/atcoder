use proconio::input;

fn main() {
    input! {
        n: usize,
        d: f64,
        h: f64,
        dhn: [(f64, f64); n],
    }

    let mut ans = 0.0f64;
    for dh in &dhn {
        let dy = h - dh.1;
        let dx = d - dh.0;
        ans = ans.max((dy / dx) * (-dh.0) + dh.1);
    }

    println!("{}", ans);
}
