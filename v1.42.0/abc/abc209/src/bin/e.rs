// 黄色 ペナ率がとても高い 実装がつめきれない

use proconio::input;
use proconio::marker::Chars;
use std::collections::{HashMap, VecDeque};

fn main() {
    input! {
        n: usize,
        mut sn: [Chars; n],
    }
    let sn_org = sn.clone();

    // sn[i] が重複しないとは書かれていない
    sn.sort_unstable();
    sn.dedup();
    let sn = sn;

    let mut id_to_word = vec![];
    let mut word_to_id = HashMap::new();
    for (i, s) in sn.iter().enumerate() {
        let ss = s.iter().collect::<String>();
        id_to_word.push(ss.clone());
        word_to_id.insert(ss, i);
    }

    // 明らかに勝ちになる単語があるので, その単語から BFS をする方針？
    // あと何語で負けるかを記憶しておき基本はその偶奇で判定, 途中で閉路ができたら Draw?
    // としても実装が重い
    let mut prev_map: HashMap<String, Vec<usize>> = HashMap::new();
    for (i, s) in sn.iter().enumerate() {
        let last_three = s.iter().skip(s.len() - 3).collect::<String>();
        if let Some(x) = prev_map.get_mut(&last_three) {
            x.push(i);
        } else {
            prev_map.insert(last_three, vec![i]);
        }
    }

    let mut is_draw = vec![false; sn.len()];
    let mut rank = vec![0; sn.len()];
    for (i, s) in sn.iter().enumerate() {
        if rank[i] != 0 {
            continue;
        }

        // let mut vdq = VecDeque::new();
    }

    for s in &sn_org {
        let i = word_to_id.get(&s.iter().collect::<String>()).unwrap();
        println!(
            "{}",
            if is_draw[*i] {
                "Draw"
            } else if rank[*i] % 2 == 0 {
                "Takahashi"
            } else {
                "Aoki"
            }
        );
    }
}
