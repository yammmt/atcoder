// :fu: 21-03 場合分け
// "AKIHABARAAAAA" は "NO"

use proconio::input;
use std::collections::HashSet;

fn bit_rows(n: u32) -> Vec<Vec<usize>> {
    let mut ret = vec![];
    for b in 0..2u64.pow(n) {
        let mut cur = vec![];
        for i in 0..n {
            if b & (1 << i) > 0 {
                cur.push(i as usize);
            }
        }
        ret.push(cur);
    }
    ret
}

fn main() {
    input! {
        s: String,
    }

    let bits = bit_rows(4);
    let mut ans = HashSet::new();
    for b in &bits {
        let mut cur = vec![];
        if b.contains(&0) {
            cur.push('A');
        }
        cur.push('K');
        cur.push('I');
        cur.push('H');
        if b.contains(&1) {
            cur.push('A');
        }
        cur.push('B');
        if b.contains(&2) {
            cur.push('A');
        }
        cur.push('R');
        if b.contains(&3) {
            cur.push('A');
        }
        ans.insert(cur.iter().collect::<String>());
    }

    println!(
        "{}",
        if ans.contains(&s) {
            "YES"
        } else {
            "NO"
        }
    );
}
