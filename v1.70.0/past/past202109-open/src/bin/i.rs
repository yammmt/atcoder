use proconio::{fastout, input};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[fastout]
fn main() {
    input! {
        n: usize,
        an: [u64; n],
    }

    // 最小値として達成可能な最大値, 作れる値について二分探索？
    // pq を用意して全部入れる, 値を取り出して 2 で割る, pq から最小値を取りだして x3?
    // 2 で割るのチェックで奇数を引き続けると辛いので pq は偶奇で２つ
    // 最悪ケースでは各項について 32 回ずつ 2 で割れる可能性があり 3x10^6 回ほど pq 操作
    // 10^8 は切れそう

    let mut available_x3 = 0;
    let mut odds = BinaryHeap::new();
    for mut a in an {
        while a % 2 == 0 {
            a /= 2;
            available_x3 += 1;
        }
        odds.push(Reverse(a));
    }

    for _ in 0..available_x3 {
        let Reverse(o) = unsafe { odds.pop().unwrap_unchecked() };
        odds.push(Reverse(3 * o));
    }

    let Reverse(ans) = unsafe { odds.pop().unwrap_unchecked() };
    println!("{ans}");
}
