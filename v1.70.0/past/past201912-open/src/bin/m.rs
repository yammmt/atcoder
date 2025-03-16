use ordered_float::OrderedFloat;
use proconio::fastout;
use proconio::input;

// こんなにいらんが
const LOOP_COUNT_MAX: usize = 100_000;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        abn: [(i64, i64); n],
        cdm: [(i64, i64); m],
    }

    let sorted_monsters = |wpn, x| {
        let mut monsters = vec![];
        for &(w, p) in wpn {
            monsters.push(OrderedFloat(p as f64 - w as f64 * x));
        }
        monsters.sort_unstable();
        monsters.reverse();
        monsters
    };

    let mut pass = 0.0;
    let mut fail = 200_001.0;
    let mut loop_cnt = 0;
    while fail - pass > 1.0e-8 {
        let mid = (pass + fail) / 2.0;

        let monsters = sorted_monsters(&abn, mid);
        let helpers = sorted_monsters(&cdm, mid);
        let wo_helpers = monsters.iter().take(5).map(|a| f64::from(*a)).sum::<f64>();
        let w_helpers =
            monsters.iter().take(4).map(|a| f64::from(*a)).sum::<f64>() + f64::from(helpers[0]);

        if wo_helpers >= 0.0 || w_helpers >= 0.0 {
            pass = mid;
        } else {
            fail = mid;
        }
        loop_cnt += 1;
        if loop_cnt >= LOOP_COUNT_MAX {
            break;
        }
    }

    println!("{pass}");
}
