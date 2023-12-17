use proconio::fastout;
use proconio::input;
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        an: [usize; n],
        bn: [usize; n],
        q: usize,
        xyq: [(usize, usize); q],
    }

    let set_len = |an: &Vec<_>| {
        let mut ret = vec![];
        let mut cur = HashSet::new();
        for a in an {
            cur.insert(*a);
            ret.push(cur.len());
        }
        ret
    };
    let a_set_len = set_len(&an);
    let b_set_len = set_len(&bn);

    let insert_order = |an: &Vec<_>| {
        let mut ret = vec![];
        let mut cur = HashSet::new();
        for a in an {
            if !cur.contains(a) {
                cur.insert(a);
                ret.push(*a);
            }
        }
        ret
    };
    let a_inserted_order = insert_order(&an);
    let b_inserted_order = insert_order(&bn);

    let mut a_b_same = vec![];
    let mut cur = HashSet::new();
    for i in 0..a_inserted_order.len().min(b_inserted_order.len()) {
        let a = a_inserted_order[i];
        let b = b_inserted_order[i];
        if cur.contains(&a) {
            cur.remove(&a);
        } else {
            cur.insert(a);
        }
        if cur.contains(&b) {
            cur.remove(&b);
        } else {
            cur.insert(b);
        }

        a_b_same.push(cur.is_empty());
    }

    for (x, y) in xyq {
        println!(
            "{}",
            if a_set_len[x - 1] == b_set_len[y - 1] && a_b_same[a_set_len[x - 1] - 1] {
                "Yes"
            } else {
                "No"
            }
        );
    }
}
