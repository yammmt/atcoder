// https://www.hamayanhamayan.com/entry/2019/12/23/220145
// 大学受験でまったく見覚えがないが忘れているだけ？時代？受けた大学のランクの差？

use proconio::input;

fn main() {
    input! {
        n: u64,
    }

    if n % 2 == 1 {
        println!("0");
        return;
    }

    let mut ans = 0;
    let mut d = 10;
    while n / d != 0 {
        ans += n / d;
        d *= 5;
    }
    println!("{}", ans);
}
