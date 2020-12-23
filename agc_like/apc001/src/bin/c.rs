// https://atcoder.jp/contests/apc001/submissions/18940872
// ↑が N 最大 (99,999) で手元では動くのに WA 引いていたのはどうして？
// ジャッジサーバーでも N = 99,997 であれば AC
// 初回に (不必要に) 半分に絞る際になにか情報が落ちている？でもそれなら手元でも動かないのでは

// 問題の性質上 panic 起こすと TLE になるらしく panic 入れてデバッグすることはできない

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

fn err_in_range(n_from: (usize, &str), n_to: (usize, &str)) -> bool {
    match (n_to.0 - n_from.0) % 2 {
        0 => n_from.1 != n_to.1,
        _ => n_from.1 == n_to.1,
    }
}

fn main() {
    let n: usize = readln();

    println!("0");
    stdout().flush().unwrap();
    let t1: String = readln();
    if t1 == "Vacant" {
        // pass
        return;
    }

    let mut ans_from = (0, t1);
    let mut ans_to = (n - 1, "".to_string());
    while ans_to.0 - ans_from.0 > 1 {
        // println!("ans_from: {:?}", ans_from);
        // println!("ans_to: {:?}", ans_to);
        let mid = (ans_from.0 + ans_to.0) / 2;
        println!("{}", mid);
        stdout().flush().unwrap();
        let t: String = readln();
        if t == "Vacant" {
            // pass
            return;
        }

        match err_in_range((ans_from.0, &ans_from.1), (mid, &t)) {
            true => ans_to = (mid, t),
            false => ans_from = (mid, t),
        }
    }
    println!("{}", ans_to.0);
    stdout().flush().unwrap();
    let _t: String = readln();
}
