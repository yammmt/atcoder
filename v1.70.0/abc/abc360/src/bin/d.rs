use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        n: usize,
        t: isize,
        s: Chars,
        xn: [isize; n],
    }

    // すれ違う = 前後で大小関係が変わる
    // 二分探索の境界苦手

    let mut to_left_begin = vec![];
    let mut to_right_begin = vec![];
    for i in 0..n {
        if s[i] == '0' {
            to_left_begin.push(xn[i]);
        } else {
            to_right_begin.push(xn[i]);
        }
    }
    to_left_begin.sort_unstable();
    to_right_begin.sort_unstable();

    let mut to_left_end = to_left_begin.clone();
    let mut to_right_end = to_right_begin.clone();
    for li in 0..to_left_end.len() {
        to_left_end[li] -= t;
    }
    for ri in 0..to_right_end.len() {
        to_right_end[ri] += t;
    }

    let mut ans = 0;
    for ri in 0..to_right_begin.len() {
        // 移動前に自身以上であるものの数 - 移動後に自身より大きいものの数
        let mut pass = -1;
        let mut fail = to_left_begin.len() as isize;
        while fail - pass > 1 {
            let mid = (pass + fail) as usize / 2;
            if to_left_begin[mid] < to_right_begin[ri] {
                pass = mid as isize;
            } else {
                fail = mid as isize;
            }
        }
        let no_less_than_begin = to_left_begin.len() - fail as usize;

        let mut pass = -1;
        let mut fail = to_left_end.len() as isize;
        while fail - pass > 1 {
            let mid = (pass + fail) as usize / 2;
            if to_left_end[mid] <= to_right_end[ri] {
                pass = mid as isize;
            } else {
                fail = mid as isize;
            }
        }
        let more_than_end = to_left_end.len() - fail as usize;

        ans += no_less_than_begin - more_than_end;
    }

    println!("{ans}");
}
