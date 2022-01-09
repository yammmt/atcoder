// これが灰前半になっている間はコンテスト出れない

use proconio::input;

fn main() {
    input! {
        mut k: u64,
    }

    let mut ans = vec![];
    while k > 0 {
        ans.push(if k % 2 == 0 { '0' } else { '2' });
        k /= 2;
    }
    ans.reverse();

    println!("{}", ans.iter().collect::<String>());
}
