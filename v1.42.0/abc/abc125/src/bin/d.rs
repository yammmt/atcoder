// 25.5min

use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [i64; n],
    }

    let mut minus_num = 0;
    let mut zero_num = 0;
    for a in &an {
        if *a < 0 {
            minus_num += 1;
        } else if *a == 0 {
            zero_num += 1;
        }
    }

    let allsum = an.iter().fold(0, |acc, x| acc + x.abs());
    if minus_num % 2 == 0 || zero_num > 0 {
        // すべての負数を反転できる
        println!("{}", allsum);
    } else {
        let mut anmin = an[0].abs();
        for a in &an {
            anmin = anmin.min(a.abs());
        }
        println!("{}", allsum - 2 * anmin);
    }
}
