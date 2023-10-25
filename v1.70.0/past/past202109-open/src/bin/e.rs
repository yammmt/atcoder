use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        k: usize,
        cn: [usize; n],
        pn: [usize; n],
    }

    let mut v = Vec::with_capacity(n);
    for i in 0..n {
        v.push((pn[i], cn[i]));
    }
    v.sort_unstable();

    let mut ans = 0;
    let mut bought_colors = HashSet::new();
    for vv in &v {
        if bought_colors.contains(&vv.1) {
            continue;
        }

        ans += vv.0;
        bought_colors.insert(vv.1);
        if bought_colors.len() == k {
            break;
        }
    }

    if bought_colors.len() < k {
        println!("-1");
        return;
    }

    println!("{ans}");
}
