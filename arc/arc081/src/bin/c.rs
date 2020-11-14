use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        an: [u64; n],
    }

    // 10^9 個の要素をもつベクトルは作れず RE になる
    let mut hm = HashMap::new();
    for a in &an {
        let cnt = hm.entry(*a).or_insert(0);
        *cnt += 1;
    }

    let mut vpair = vec![];
    for (k, v) in &hm {
        if *v < 2 {
            continue;
        }

        vpair.push((*k, *v));
    }
    if vpair.is_empty() {
        println!("0");
        return;
    }

    vpair.sort_unstable();
    let ans = if vpair[vpair.len() - 1].1 >= 4 {
        vpair[vpair.len() - 1].0 * vpair[vpair.len() - 1].0
    } else {
        vpair[vpair.len() - 1].0 * vpair[vpair.len() - 2].0
    };
    println!("{}", ans);
}
