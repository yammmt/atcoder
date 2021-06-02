// 10min
// ちょっともたつく

use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [usize; n],
        bn: [usize; n],
        cn: [usize; n],
    }

    let mut a46 = vec![0; 46];
    for a in &an {
        a46[*a % 46] += 1;
    }
    let mut b46 = vec![0; 46];
    for b in &bn {
        b46[*b % 46] += 1;
    }
    let mut c46 = vec![0; 46];
    for c in &cn {
        c46[*c % 46] += 1;
    }

    let mut ans = 0usize;
    for ai in 0..46 {
        if a46[ai] == 0 {
            continue;
        }

        for bi in 0..46 {
            if b46[bi] == 0 {
                continue;
            }

            let ci = (46 + 46 + 46 - (ai + bi)) % 46;
            ans += a46[ai] * b46[bi] * c46[ci];
        }
    }

    println!("{}", ans);
}
