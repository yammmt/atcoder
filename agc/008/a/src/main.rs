// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        x: i64,
        y: i64,
    }

    let mut ans = std::i64::MAX;
    if x <= y {
        // 最初にも最後にも B を押さない
        ans = ans.min(y - x);
    }
    if -x <= y {
        // 最初に B を押し最後に B を押さない
        ans = ans.min(y + x + 1);
    }
    if x <= -y {
        // 最初に B を押さず最後に B を押す
        ans = ans.min(-y - x + 1);
    }
    if -x <= -y {
        // 最初にも最後にも B を押す
        ans = ans.min(-y + x + 2);
    }
    println!("{}", ans);
}
