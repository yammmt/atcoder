use proconio::fastout;
use proconio::input;
use std::collections::VecDeque;

#[fastout]
fn main() {
    // ヒストグラム中の最大長方形

    input! {
        mut n: usize,
        mut an: [usize; n],
    }
    an.push(0);
    n += 1;

    let mut ans = 0;
    let mut stack: VecDeque<(usize, usize)> = VecDeque::new();
    for i in 0..n {
        let mut left = i;
        while let Some(a_cur) = stack.pop_back() {
            if a_cur.0 > an[i] {
                left = a_cur.1;
                let length = i - left;
                ans = ans.max(a_cur.0 * length);
            } else {
                stack.push_back(a_cur);
                break;
            }
        }
        stack.push_back((an[i], left));
    }

    println!("{ans}");
}
