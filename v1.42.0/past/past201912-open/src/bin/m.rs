// https://www.youtube.com/watch?v=oPyaHk7KjTw
// 貪欲では解にならない場合がある

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        abn: [(f64, f64); n],
        cdm: [(f64, f64); m],
    }

    let mut pass = 0.0;
    let mut fail = 100_000.0 * 6.0;
    while fail - pass > 1e-8 {
        let mid = (fail + pass) / 2.0;

        let mut ax = vec![];
        for i in 0..n {
            ax.push(abn[i].1 - mid * abn[i].0);
        }
        ax.sort_unstable_by(|a, b| a.partial_cmp(&b).unwrap());

        let mut cx = vec![];
        for i in 0..m {
            cx.push(cdm[i].1 - mid * cdm[i].0);
        }
        let mut maxcx = cx[0];
        for c in &cx {
            maxcx = maxcx.max(*c);
        }

        let mut pts = 0.0;
        for i in 0..4 {
            pts += ax[n - 1 - i];
        }
        pts += ax[n - 1 - 4].max(maxcx);

        if pts < 0.0 {
            fail = mid;
        } else {
            pass = mid;
        }
    }

    println!("{}", pass);
}
