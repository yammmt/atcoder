// 15min

use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: u64,
        m: usize,
        an: [u64; n],
        bcm: [(u64, u64); m],
    }

    let mut hm = HashMap::new();
    for a in &an {
        let cnt = hm.entry(*a).or_insert(0);
        *cnt += 1;
    }
    let mut cards = vec![];
    for (k, v) in &hm {
        cards.push((*k, *v));
    }

    for bc in &bcm {
        cards.push((bc.1, bc.0));
    }

    // unstable sort だと 47ms
    // stable sort だと 58ms
    cards.sort_unstable();
    // println!("cards: {:?}", cards);
    let mut ans = 0u64;
    let mut selected = 0;
    let mut idx = cards.len() - 1;
    while selected < n {
        let cselected = (cards[idx].1).min(n - selected);
        ans += cards[idx].0 * cselected;
        selected += cselected;
        if selected < n {
            idx -= 1;
        }
        // println!("ans: {}", ans);
    }
    println!("{}", ans);
}
