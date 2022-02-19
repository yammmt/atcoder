// FIXME: 二分探索やるだけでは通らない

use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        an: [usize; n],
    }

    let mut pass = 0;
    let mut fail = l + 1;
    while fail - pass > 1 {
        // 切断後の一番短い棒の長さを mid 以上にできるか？
        let mid = (pass + fail) / 2;
        // println!("{} {} => {}", pass, fail, mid);

        let mut cur = 0;
        let mut cur_pass = true;
        let mut prev_len = 0;
        let mut min_len = l + 1;
        for a in &an {
            // println!("  {}, cur: {}, prev: {}", a, cur, prev_len);
            if cur + *a >= mid {
                // 長さが mid 以上になったので切る
                // println!("    cut");
                prev_len = if *a >= mid { *a } else { cur };

                if cur + *a > l {
                    // mid 以上になると共に元の制約に反すので除外
                    cur_pass = false;
                    break;
                }

                min_len = min_len.min(cur + *a);
                cur = 0;
            } else {
                cur += *a;
            }
        }
        // 最後の端数分を前の分割に入れて L を超えるか否かを見る
        // println!("  prev_len: {}", prev_len);
        if cur != 0 {
            min_len = min_len.min(prev_len + cur);
            if prev_len + cur > l {
                cur_pass = false;
            }
        }

        // println!("  {}, {:?}", min_len, cur_pass);
        if cur_pass && min_len >= mid {
            // println!("  pass");
            pass = mid;
        } else {
            // println!("  fail");
            fail = mid;
        }
    }

    println!("{}", pass);
}
