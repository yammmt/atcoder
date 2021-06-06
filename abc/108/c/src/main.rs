// :fu: 21-06 緑の範疇だが苦手

// https://pyteyon.hatenablog.com/entry/2018/09/02/094228

use proconio::input;

fn main() {
    input! {
        n: u64,
        k: u64,
    }

    let mut ans = (n / k).pow(3);
    if k % 2 == 0 {
        ans += if n % k >= k / 2 {
            (n / k + 1).pow(3)
        } else {
            (n / k).pow(3)
        };
    }

    println!("{}", ans);
}
