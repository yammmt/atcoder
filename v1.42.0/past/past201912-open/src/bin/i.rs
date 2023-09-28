// bitDP
// 買う買わない全列挙すると 2^1000 となり到底無理

use proconio::input;

fn char_to_nsize(c: &Vec<char>) -> usize {
    let mut ret = 0;
    let mut two = 1;
    for i in c {
        if *i == 'Y' {
            ret += two;
        }
        two *= 2;
    }
    ret
}

fn main() {
    input! {
        n: usize,
        m: usize,
        scm: [(String, i64); m],
    }

    let mut scm_new = vec![];
    for sc in &scm {
        let vc = (sc.0).chars().collect::<Vec<char>>();
        let curset = char_to_nsize(&vc);
        scm_new.push((curset, sc.1));
    }
    // println!("{:?}", scm_new);

    let max_idx = 2u64.pow(n as u32) as usize;
    let mut dp = vec![std::i64::MAX; max_idx + 1];
    dp[0] = 0;
    for i in 0..max_idx + 1 {
        if dp[i] == std::i64::MAX {
            // no routes to reach
            continue;
        }

        for sc in &scm_new {
            let next_idx = i as usize | sc.0;
            if next_idx > max_idx {
                continue;
            }

            dp[next_idx] = dp[next_idx].min(dp[i] + sc.1);
        }
    }
    // println!("{:?}", dp);

    println!(
        "{}",
        if dp[max_idx - 1] == std::i64::MAX {
            -1
        } else {
            dp[max_idx - 1]
        }
    );
}
