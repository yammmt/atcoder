// 17.5min
// cur のゼロクリア漏れ

use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [i64; n],
    }

    let mut pm = 0;
    let mut cur = 0;
    for (i, a) in an.iter().enumerate() {
        cur += *a;
        // - -> + or + -> -
        if (i % 2 == 0 && cur <= 0) || (i % 2 == 1 && cur >= 0) {
            pm += cur.abs() + 1;
            cur = if i % 2 == 0 {
                1
            } else {
                -1
            };
        }
    }
    // println!("{}", pm);

    let mut mp = 0;
    let mut cur = 0;
    for (i, a) in an.iter().enumerate() {
        cur += *a;
        // + -> - or - -> +
        if (i % 2 == 0 && cur >= 0) || (i % 2 == 1 && cur <= 0) {
            mp += cur.abs() + 1;
            cur = if i % 2 == 0 {
                -1
            } else {
                1
            };
        }
    }
    // println!("{}", mp);

    println!("{}", pm.min(mp));
}
