// 15.5min (1WA: 12.5min)
// WA: 出力形式 (RE も混じる)

use std::io::{stdout, Write};

fn get_line() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().to_string()
}

fn readln<T>() -> T
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug {
    get_line().parse().unwrap()
}

fn main() {
    let n: usize = readln();
    // 2 <= n <= 50 より辺数 m は 1 <= m <= 49
    // 愚直に 50C2 = 1,225 回質問すれば全点間の距離が求められる
    // 満点のためには質問回数の上限は 100 回

    // 頂点 0 から全頂点への距離を尋ねる
    // 最も遠かった点から再度全頂点への距離を訪ねて最長だったものが答え
    // 質問回数は高々 49 x 2 回
    let mut from_0_max = (0, 0);
    for i in 2..n + 1 {
        println!("? 1 {}", i);
        stdout().flush().unwrap();

        let cur: usize = readln();
        if cur > from_0_max.0 {
            from_0_max = (cur, i);
        }
    }

    let mut ans = 0;
    let base = from_0_max.1;
    for i in 1..n + 1 {
        if i == base {
            continue;
        }

        println!("? {} {}", base, i);
        stdout().flush().unwrap();

        let cur: usize = readln();
        ans = ans.max(cur);
    }

    println!("! {}", ans);
}
