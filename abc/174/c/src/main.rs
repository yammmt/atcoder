// See https://qiita.com/tanakh/items/a312a9bd684658ab1e7b#c---repsept

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        k: u64,
    }

    let mut ans = 1;
    let mut seven = 7;
    let mut appear_set = HashSet::new();
    loop {
        let cur = seven % k;
        if cur == 0 {
            println!("{}", ans);
            return;
        } else if appear_set.contains(&cur) {
            println!("-1");
            return;
        }
        appear_set.insert(cur);
        seven = cur * 10 + 7;
        ans += 1;
    }
}
