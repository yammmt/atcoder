// 23.5min PAST で見た
// 小数の精度に言及がないけれど大丈夫なのだろうか

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        wpn: [(f64, f64); n],
    }

    let mut wsn = vec![];
    for wp in &wpn {
        wsn.push((wp.0, wp.0 * wp.1 / 100.0));
    }

    let mut cnt = 0;
    let mut pass = 0.0;
    let mut fail = 1.0;
    while fail - pass > 1e-8 && cnt < 1_000_000 {
        let mid = (fail + pass) / 2.0;

        let mut vpts = vec![];
        for (i, ws) in wsn.iter().enumerate() {
            vpts.push((ws.1 - ws.0 * mid, i));
        }
        vpts.sort_by(|a, b| a.partial_cmp(&b).unwrap());
        vpts.reverse();

        let mut solt = 0.0;
        let mut total = 0.0;
        for pts in vpts.iter().take(k) {
            solt += wsn[pts.1].1;
            total += wsn[pts.1].0;
        }
        if solt / total >= mid {
            pass = mid;
        } else {
            fail = mid;
        }

        cnt += 1;
    }

    println!("{}", pass * 100.0);
}
