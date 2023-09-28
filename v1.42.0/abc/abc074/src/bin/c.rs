// 数問

use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
        e: i64,
        f: i64,
    }

    // 水/砂糖量全部列挙する
    let amax = f / (100 * a) + 1;
    let bmax = f / (100 * b) + 1;
    let cmax = f / c + 1;
    let dmax = f / d + 1;
    let mut ans_water = 0;
    let mut ans_suger = 0;
    for i in 0..cmax {
        for j in 0..dmax {
            let cur_suger =  c * i + d * j;
            let required_water = ((cur_suger + e - 1) / e) * 100;
            if cur_suger + required_water > f {
                break;
            }

            for ii in 0..amax {
                for jj in 0..bmax {
                    let cur_water = 100 * a * ii + 100 * b * jj;
                    if cur_water < required_water || cur_suger + cur_water > f {
                        continue;
                    }

                    if cur_suger * (ans_water + ans_suger) >= ans_suger * (cur_water + cur_suger) {
                        ans_water = cur_water;
                        ans_suger = cur_suger;
                    }
                }
            }
        }
    }

    println!("{} {}", ans_water + ans_suger, ans_suger);
}
