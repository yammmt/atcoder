use proconio::input;
// use std::collections::VecDeque;

// 硬貨を袋に戻すのに取り出す確率変わらないの？

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
    }

    let a_per = (a as f64) / ((a + b + c) as f64);
    let b_per = (b as f64) / ((a + b + c) as f64);
    let c_per = (c as f64) / ((a + b + c) as f64);

    let mut ans = 0.0;
    for i in a..101 {
        let a_diff = i - a;
        for j in b..101 {
            let b_diff = j - b;
            for k in c..101 {
                if (i != 100 && j != 100 && k != 100) || (i == 100 && j == 100) || (i == 100 && k == 100) || (j == 100 && k == 100) {
                    continue;
                }

                let c_diff = k - c;
                // println!("{} {} {}", i, j, k);
                let mut cur_per = 1.0;
                if a_diff > 0 {
                    cur_per *= (a_diff as f64) * a_per;
                }
                if b_diff > 0 {
                    cur_per *= (b_diff as f64) * b_per;
                }
                if c_diff > 0 {
                    cur_per *= (c_diff as f64) * c_per;
                }
                // println!("cur_per: {}", cur_per);

                ans += ((a_diff + b_diff + c_diff) as f64) * cur_per;
            }
        }
    }

    // let mut vdq = VecDeque::new();
    // vdq.push_back((1, 1.0, (a, b, c)));
    // let mut ans = 0.0;
    // // TLE
    // // let mut ith = 1;
    // while !vdq.is_empty() {
    //     let cur = vdq.pop_front().unwrap();
    //     // println!("{}: {:?}", ith, cur);
    //     // ith += 1;

    //     // a を引く
    //     if a != 0 {
    //         let p = cur.1 * (cur.2.0 as f64 / ((cur.2.0+cur.2.1+cur.2.2) as f64));
    //         if cur.2.0 == 99 {
    //             ans += cur.0 as f64 * p;
    //         } else {
    //             vdq.push_back((cur.0 + 1, p, (cur.2.0 + 1, cur.2.1, cur.2.2)));
    //         }
    //     }
    //     // b を引く
    //     if b != 0 {
    //         let p = cur.1 * (cur.2.1 as f64 / ((cur.2.0+cur.2.1+cur.2.2) as f64));
    //         if cur.2.1 == 99 {
    //             ans += cur.0 as f64 * p;
    //         } else {
    //             vdq.push_back((cur.0 + 1, p, (cur.2.0, cur.2.1 + 1, cur.2.2)));
    //         }
    //     }
    //     // c を引く
    //     if c != 0 {
    //         let p = cur.1 * (cur.2.2 as f64 / ((cur.2.0+cur.2.1+cur.2.2) as f64));
    //         if cur.2.2 == 99 {
    //             ans += cur.0 as f64 * p;
    //         } else {
    //             vdq.push_back((cur.0 + 1, p, (cur.2.0, cur.2.1, cur.2.2 + 1)));
    //         }
    //     }
    // }
    println!("{}", ans);
}
