use proconio::input;
// use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        fnn: [[i64; 10]; n],
        pn: [[i64; 11]; n],
    }
    // println!("{:?}", pn);

    let mut ans = std::i64::MIN;
    for bit_row in 1..2u32.pow(10u32) {
        let mut opened = vec![];
        for i in 0..10 {
            if bit_row & (1 << i) > 0 {
                opened.push(i);
            }
        }

        let mut cur = 0;
        for (i, f) in fnn.iter().enumerate() {
            let mut common = 0;
            for o in &opened {
                if f[*o as usize] == 1 {
                    common += 1;
                }
            }
            cur += pn[i][common];
        }
        ans = ans.max(cur);
    }

    println!("{}", ans);
}
