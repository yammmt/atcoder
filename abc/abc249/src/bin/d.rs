// :fu: :fu: :fu: 22-04 数問 強めの誤読
// 高速約数列挙を入れると WA や TLE が出て詰み 方針はあってるはずだが

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut an: [usize; n],
    }
    // an.sort();
    // println!("{:?}", an);

    let mut cnt = vec![0; 200_001];
    for a in &an {
        cnt[*a] += 1;
    }

    let mut ans = 0u64;
    for &a in &an {
        let mut divisors = vec![];
        let mut p = 1;
        while p * p <= a {
            if a % p == 0 {
                divisors.push(p);
                if a / p != p {
                    divisors.push(a / p);
                }
            }
            p += 1;
        }

        for &d in &divisors {
            ans += cnt[d] * cnt[a / d];
        }
    }

    println!("{}", ans);
}
