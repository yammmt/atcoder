// :fu: 21-05 BTreeSet で重実装寄り
// 操作ややこしくサンプルが通っているとデバッグつらい
// BTreeSet は multiset ではない

use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        q: usize,
        abn: [(usize, usize); n],
        cdq: [(usize, usize); q],
    }

    // ordered set (BTreeSet) で全幼稚園のレート最高を記憶
    // first/last が nightly 状態なので最大値最小値がだるい

    let mut kgs = vec![BTreeSet::new(); 200_000];
    let mut infants = vec![0; n];
    for (i, ab) in abn.iter().enumerate() {
        kgs[ab.1 - 1].insert((ab.0, i));
        infants[i] = ab.1 - 1;
    }

    let mut ans_candidates = BTreeSet::new();
    for kg in &kgs {
        if !kg.is_empty() {
            ans_candidates.insert(*kg.iter().last().unwrap());
        }
    }

    for cd in &cdq {
        // 幼児の所属を移動
        let moved_from = infants[cd.0 - 1];
        let moved_to = cd.1 - 1;
        let moved_data = (abn[cd.0 - 1].0, cd.0 - 1);
        infants[cd.0 - 1] = moved_to;

        // 出力値の更新: 移動元
        let moved_from_max = *kgs[moved_from].iter().last().unwrap();
        kgs[moved_from].remove(&moved_data);
        if moved_from_max == moved_data {
            // kgs[moved_from] が移動元の最大値であれば, 移動した幼児の値を最大値集合から消す
            ans_candidates.remove(&moved_data);
            if let Some(cur) = kgs[moved_from].iter().last() {
                // 移動元の最大値を入れ直す
                ans_candidates.insert(*cur);
            }
        }

        // 出力値の更新: 移動先
        if let Some(moved_to_max) = kgs[moved_to].iter().last() {
            if moved_to_max.0 < abn[cd.0 - 1].0 {
                // 移動先の幼稚園で最大値となった
                ans_candidates.remove(&moved_to_max);
                ans_candidates.insert(moved_data);
            }
        } else {
            // 幼稚園の唯一の幼児であり最大値集合に追加
            ans_candidates.insert(moved_data);
        }
        kgs[moved_to].insert(moved_data);

        println!("{}", ans_candidates.iter().next().unwrap().0);
    }
}
