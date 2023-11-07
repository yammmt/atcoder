use proconio::fastout;
use proconio::input;
use std::collections::BTreeSet;

static DUMMY: u64 = u64::MAX / 4;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: u64,
        b: u64,
        c: u64,
        dnn: [[u64; n]; n],
    }

    // editorial のように電車で移動 -> 社用車に移動, と図面を分けた方が賢いかも

    let mut by_car = vec![DUMMY; n];
    let mut by_train = vec![DUMMY; n];
    // (総移動時間, now, car?)
    let mut bts = BTreeSet::new();
    bts.insert((0, 0, true));
    while let Some(cur) = bts.pop_first() {
        if cur.2 == true && by_car[cur.1] == DUMMY {
            // 車で始めて到着した
            // 車で来たので車で動ける
            by_car[cur.1] = cur.0;
            for i in 0..n {
                if by_car[i] == DUMMY {
                    bts.insert((cur.0 + dnn[cur.1][i] * a, i, true));
                }
            }
        }
        // 電車に乗り換えるのは前の手段に依らない
        if by_train[cur.1] == DUMMY {
            by_train[cur.1] = cur.0;
            for i in 0..n {
                if by_train[i] == DUMMY {
                    bts.insert((cur.0 + dnn[cur.1][i] * b + c, i, false));
                }
            }
        }
    }

    println!("{}", by_car[n - 1].min(by_train[n - 1]));
}
