// :fu: :fu: :fu: :fu: :fu: 21-12 数問 とても嫌い 解説の行間で詰み
// https://twitter.com/kyopro_friends/status/1466766241839075328/photo/1
// サンプルはエッジケースなりうるものを含んでおり親切

use proconio::input;

fn main() {
    input! {
        n: u64,
    }

    let mut ans = 0;
    // let mut i = 1;

    // // 愚直に数え上げる
    // // n / i = j
    // while i * i <= n {
    //     ans += n / i;
    //     i += 1;
    // }
    // // println!("i: {}", i);
    // // println!("  ans: {}", ans);

    // // 答えが j となる数はいくつあるか
    // // n / j = i
    // for j in 1..i {
    //     if j == n / j {
    //         // j * j == n では WA (ex. n = 10, j = 3)
    //         // println!("continue: {}", j);
    //         continue;
    //     }

    //     // println!("j: {}", j);
    //     // println!("  ans += {}", j * (n / j - n / (j + 1)));
    //     ans += j * (n / j - n / (j + 1));
    // }

    let mut i = 1;
    while i <= n {
        let x = n / i;
        let ni = n / x + 1;
        println!("i: {}", i);
        println!("x: {}", x);
        println!("ni: {}", ni);
        println!();
        ans += x * (ni - i);
        i = ni;
    }

    println!("{}", ans);
}
