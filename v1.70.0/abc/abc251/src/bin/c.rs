use proconio::fastout;
use proconio::input;
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        stn: [(String, isize); n],
    }

    let mut counter = HashMap::new();
    let mut originals = vec![];
    for (i, (s, t)) in stn.iter().enumerate() {
        let cnt = counter.entry(s).or_insert(0);
        *cnt += 1;
        if *cnt == 1 {
            // (点数, i 番目)
            // 提出が早いものをソートすべく点数は符号反転
            originals.push((-t, i + 1));
        }
    }
    // println!("{:?}", counter);

    originals.sort_unstable();

    println!("{}", originals[0].1);
}
