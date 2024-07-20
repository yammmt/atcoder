use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

const DUMMY: usize = usize::MAX / 4;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        scm: [(Chars, usize); m],
    }

    let mut ucm = vec![];
    for (s, c) in scm {
        let mut b = 0;
        for i in 0..n {
            b <<= 1;
            if s[i] == 'Y' {
                b |= 1;
            }
        }
        ucm.push((b, c));
    }

    let i_max = 2usize.pow(n as u32);
    let mut dp = vec![DUMMY; i_max];
    dp[0] = 0;
    for i in 0..i_max {
        if dp[i] == DUMMY {
            continue;
        }

        for j in 0..m {
            let i_next = i | ucm[j].0;
            dp[i_next] = dp[i_next].min(dp[i] + ucm[j].1);
        }
    }

    if dp[i_max - 1] == DUMMY {
        println!("-1");
        return;
    }

    println!("{}", dp[i_max - 1]);
}
