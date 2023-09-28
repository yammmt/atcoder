// -*- coding:utf-8-unix -*-

// 32min
// WA: ans のオーバーフロー

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut ans = 0u64;
    let mut a_streak = 0u64;
    let mut prev_b = false;
    for c in &s {
        match *c {
            'A' => {
                if prev_b {
                    // "BA" なら遡って消せなくなるのでリセット
                    a_streak = 1;
                } else {
                    a_streak += 1;
                }
                prev_b = false;
            },
            'B' => {
                if prev_b {
                    a_streak = 0;
                }
                prev_b = true;
            },
            'C' => {
                if prev_b && a_streak > 0 {
                    ans += a_streak;
                } else {
                    a_streak = 0;
                }
                prev_b = false;
            },
            _ => unreachable!()
        }
    }
    println!("{}", ans);
}
