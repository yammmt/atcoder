// See https://qiita.com/tanakh/items/a312a9bd684658ab1e7b#c---repsept

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        k: u64,
    }

    let mut seven = 7;
    let mut idx = 1;
    let mut appeared = HashSet::new();
    while !appeared.contains(&seven) {
        if seven % k == 0 {
            println!("{}", idx);
            return;
        }

        appeared.insert(seven);
        seven = 10 * seven + 7;
        seven %= k;
        idx += 1;
    }

    println!("-1");
}
