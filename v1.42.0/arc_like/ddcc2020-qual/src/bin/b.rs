// 8.5min

use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [i64; n],
    }

    let mut lsum = 0;
    let mut rsum = an.iter().sum::<i64>();
    let mut ans = std::i64::MAX;
    // 棒 i の後ろで切る
    for i in &an {
        lsum += *i;
        rsum -= *i;
        if lsum == rsum {
            println!("0");
            return;
        }

        let mut rrsum = rsum;
        if rrsum % 2 == 1 {
            if lsum < rsum {
                rrsum -= 1;
            } else {
                rrsum += 1;
            }
        }
        ans = ans.min((lsum - rrsum).abs() + rsum % 2);
    }
    println!("{}", ans);
}
