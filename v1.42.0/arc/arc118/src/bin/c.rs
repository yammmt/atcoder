// :fu: :fu: 21-05 数問
// 入力が数字一つで済むので提出前に最大ケースを見ておくこと

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
    }

    // 素数を N 個探して全項から一つずつ割るとオーバーフロー
    // あるいは全数 [2, 10000] を重複なしに数えれば出るが明らかに TLE

    let base = vec![6, 10, 15];
    let mut ans = HashSet::new();
    ans.insert(6);
    ans.insert(10);
    ans.insert(15);
    let mut base_idx = 0;
    let mut mul_idx = 2;
    while ans.len() < n {
        ans.insert(mul_idx * base[base_idx]);

        mul_idx += 1;
        if mul_idx * base[base_idx] > 10000 {
            base_idx += 1;
            mul_idx = 2;
        }
    }

    for (i, a) in ans.iter().enumerate() {
        print!("{}", a);
        if i == n - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
}
