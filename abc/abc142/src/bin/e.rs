// bitDP

use proconio::input;

const UNUSED: usize = std::usize::MAX;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut abcm = vec![];
    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
        }
        input! {
            cb: [usize; b],
        }
        let mut cur = 0;
        cb.iter().for_each(|c| cur |= 1 << (*c - 1));
        abcm.push((a, b, cur));
    }
    // println!("{:?}", abcm);

    let bit_row_max = (2u32.pow(n as u32) - 1) as usize;
    let mut dp = vec![UNUSED; bit_row_max + 1];
    dp[0] = 0;
    for abc in &abcm {
        for i in 0..dp.len() {
            if dp[i] == UNUSED {
                continue;
            }

            let next_i = i | abc.2;
            dp[next_i] = dp[next_i].min(dp[i] + abc.0);
        }
    }

    println!(
        "{}",
        if dp[bit_row_max] == UNUSED {
            -1
        } else {
            dp[bit_row_max] as isize
        }
    );
}
