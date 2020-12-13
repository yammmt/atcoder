// -*- coding:utf-8-unix -*-

// 22.5min 1WA (15min) (+ 腹痛軟禁)
// WA: "すべての人の発言が正しい" の解釈違い 質問欄に公開されていた

use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [usize; n],
    }
    let d = 10u64.pow(9) + 7;

    let mut ans = 1u64;
    let mut vnum = vec![0; 3];
    for a in &an {
        let mut candidates = vec![];
        for i in 0..3 {
            if vnum[i] == *a {
                candidates.push(i);
            }
        }
        if candidates.len() == 0 {
            println!("0");
            return;
        }

        vnum[candidates[0]] += 1;
        ans = (ans * candidates.len() as u64) % d;
    }
    println!("{}", ans);
}
