// :fu: 21-03

// 茶は撤退補正の結果に見える (ABC 換算だと水色中盤？)
// https://twitter.com/shobonvip/status/1368729835644723201

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            _s3: [Chars; 3],
        }

        let mut ans = vec!['0'; n];
        for _ in 0..n {
            ans.push('1');
        }
        ans.push('0');

        println!("{}", ans.iter().collect::<String>());
    }
}
