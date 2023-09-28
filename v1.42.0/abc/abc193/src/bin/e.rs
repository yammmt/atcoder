// NOT WORK
// 難易度高めなので後回しにする

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        t: usize,
        xypqt: [(i64, i64, i64, i64); t],
    }

    for xypq in &xypqt {
        let mut pass = false;
        let x = xypq.0;
        let y = xypq.1;
        let p = xypq.2;
        let q = xypq.3;
        let mut hs = HashSet::new();
        let mut t_idx = 0;
        let mut w_idx = 0;

        // [b, e)
        let mut train_stay = (x, x + y);
        let mut wake_up = (p, p + q);
        loop {
            if (wake_up.0 >= train_stay.0 && wake_up.0 < train_stay.1)
                || (train_stay.0 >= wake_up.0 && train_stay.0 < wake_up.1)
            {
                println!("{}", wake_up.0.max(train_stay.0));
                pass = true;
                break;
            }

            // ?
            let delay = wake_up.0 - train_stay.0;
            if hs.contains(&delay) {
                break;
            }
            hs.insert(delay);

            while train_stay.1 < wake_up.1 {
                t_idx += 1;
                let txy = 2 * x + 2 * y;
                train_stay = (txy * t_idx + x, txy * t_idx + x + y);
            }
            while wake_up.1 < train_stay.1 {
                w_idx += 1;
                wake_up = ((p + q) * w_idx + p, (p + q) * (w_idx + 1));
            }
        }

        if !pass {
            println!("infinity");
        }
    }
}
