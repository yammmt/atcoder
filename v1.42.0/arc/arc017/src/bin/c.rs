use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        x: i64,
        front: [i64; (n + 1) / 2],
        rear: [i64; n / 2],
    }

    let mut front_set = HashMap::new();
    front_set.insert(0, 1);
    for f in &front {
        let mut next = HashMap::new();
        for (k, v) in &front_set {
            // 選ばない
            let cnt = next.entry(*k).or_insert(0);
            *cnt += v;
            // 選ぶ
            let cnt = next.entry(k + *f).or_insert(0);
            *cnt += v;
        }
        front_set = next;
    }

    let mut rear_set = HashMap::new();
    rear_set.insert(0, 1);
    for r in &rear {
        let mut next = HashMap::new();
        for (k, v) in &rear_set {
            // 選ばない
            let cnt = next.entry(*k).or_insert(0);
            *cnt += v;
            // 選ぶ
            let cnt = next.entry(k + *r).or_insert(0);
            *cnt += v;
        }
        rear_set = next;
    }

    let mut ans = 0;
    for (k, v) in &front_set {
        if let Some(cur) = rear_set.get_key_value(&(x - *k)) {
            ans += *v * cur.1;
        }
    }

    println!("{}", ans);
}
