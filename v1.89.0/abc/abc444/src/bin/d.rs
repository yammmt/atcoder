use itertools::Itertools;
use proconio::fastout;
use proconio::input;

const AMAX: usize = 200_002;

#[fastout]
fn main() {
    input! {
        n: usize,
        an: [usize; n],
    }

    let mut imos = vec![0; AMAX];
    for a in an {
        // 自明だが便宜上
        imos[0] += 1;
        imos[a] -= 1;
    }

    let mut ans = vec![];
    let mut cur = 0;
    let mut carry = 0;
    for i in 0..AMAX {
        cur += imos[i];
        let v = cur + carry;
        ans.push(v % 10);
        carry = v / 10;
    }
    // 最終桁の繰り上がり
    while carry > 0 {
        let v = carry;
        ans.push(v % 10);
        carry = v / 10;
    }

    // ちゃんと実装すれば回避できそう
    while let Some(0) = ans.last() {
        ans.pop();
    }
    ans.reverse();

    println!("{}", ans.iter().join(""));
}
