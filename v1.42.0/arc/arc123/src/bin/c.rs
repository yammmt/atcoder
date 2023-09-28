// NOT WORK

use proconio::input;
// use std::collections::HashSet;
// use std::collections::HashMap;

#[allow(clippy::collapsible_if)]
fn ai_no_more_than_n(mut n: u64) -> u64 {
    let mut vn = vec![];
    while n > 0 {
        vn.push(n % 10);
        n /= 10;
    }
    vn.reverse();

    // n の中に 0 が含まれる場合は 0 の直前で小さい数にすり替える
    let mut vc = vec![];
    let mut is_small = false;
    for (i, nn) in vn.iter().enumerate() {
        if is_small {
            vc.push(3);
        } else {
            if i != vn.len() - 1 && vn[i + 1] == 0 {
                is_small = true;
                vc.push((*nn - 1).min(3));
            } else if *nn > 3 {
                is_small = true;
                vc.push(3);
            } else {
                vc.push(*nn);
            }
        }
    }

    let mut ret = 0;
    for c in &vc {
        ret *= 10;
        ret += *c;
    }

    ret
}

fn main() {
    input! {
        t: usize,
        nt: [u64; t],
    }

    // 10^18 以下の各 A_i の候補は 3^17 個以上ある
    // 愚直に候補全部もって map 作っていくと TLE するし MLE もしそう
    // 1 が使えるのでどの数も最大で K は N と一致する
    // 引けるだけ引く, では N = 91 が反例と思いきや (33, 33, 22, 3) も解
    // とりあえず大きい数から引いていくとサンプルは全部通るがほぼ WA
    // 貪欲では N = 43 に対して (32, 11) が算出できない
    for n in &nt {
        let mut nn = *n;
        let mut ans = 0;
        while nn > 0 {
            nn -= ai_no_more_than_n(nn);
            ans += 1;
        }
        println!("{}", ans);
    }
}
