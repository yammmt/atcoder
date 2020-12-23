// TLE
// 組み合わせ次第で結果が変わるので DP ではない
// 買う買わない全列挙すると 2^1000 となり到底無理

use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        scm: [(Chars, i64); m],
    }

    let mut cheapest = HashMap::new();
    let mut vdq = VecDeque::new();
    let mut could_buy = vec![false; n];
    for i in 0..m {
        let mut bought = vec![false; n];
        for j in 0..n {
            if scm[i].0[j] == 'Y' {
                could_buy[j] = true;
                bought[j] = true;
            }
        }
        let boughtstr = bought
            .iter()
            .map(|t|
                if *t {
                    'Y'
                } else {
                    'N'
                }
            )
            .collect::<String>();
        let cnt = cheapest.entry(boughtstr.clone()).or_insert(std::i64::MAX);
        if scm[i].1 < *cnt {
            vdq.push_back((boughtstr, scm[i].1));
            *cnt = scm[i].1;
        }
    }
    if could_buy.iter().any(|a| !a) {
        println!("-1");
        return;
    }

    let mut ans = std::i64::MAX;
    while let Some(cur) = vdq.pop_back() {
        let already_bought = (cur.0)
            .chars()
            .map(|a| a == 'Y')
            .collect::<Vec<bool>>();
        if already_bought.iter().all(|&a| a) {
            ans = ans.min(cur.1);
            continue;
        }

        for i in &scm {
            for j in 0..n {
                if (i.0)[j] == 'Y' && !already_bought[j] {
                    let mut curbought = cur.0.clone().chars().collect::<Vec<char>>();
                    for k in 0..n {
                        if (i.0)[k] == 'Y' {
                            curbought[k] = 'Y';
                        }
                    }
                    let curbought_str = curbought.iter().collect::<String>();
                    let cnt = cheapest.entry(curbought_str.clone()).or_insert(std::i64::MAX);
                    if cur.1 + i.1 < *cnt {
                        vdq.push_back((curbought_str, cur.1 + i.1));
                        *cnt = cur.1 + i.1;
                    }
                    break;
                }
            }
        }
    }

    println!("{}", ans);
}
