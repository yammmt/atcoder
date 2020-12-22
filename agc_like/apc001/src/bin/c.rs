// WA 出る 1_27.txt/1_28.txt は手元だと 17/18 回で AC するが？
// 1_20.txt 見ると N = 99,997 だと通り 99,999 だと通らないようだが
// mid 計算でオーバーフロー起こす数でもなく謎
// WA 出る三問はいずれも N 最大 (99,999)
// 問題の性質上 panic 起こすと TLE になる？

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

    println!("{}", n / 2);
    stdout().flush().unwrap();
    let t2: String = readln();
    if t2 == "Vacant" {
        // pass
        return;
    }

    let mut ans_from;
    let mut ans_to;
    if err_in_range((0, &t1), (n / 2, &t2)) {
        ans_from = (0, t1);
        ans_to = (n / 2, t2);
    } else {
        ans_from = (n / 2, t1);
        ans_to = (n, t2);
    }

    while ans_to.0 - ans_from.0 > 1 {
        println!("ans_from: {:?}", ans_from);
        println!("ans_to: {:?}", ans_to);
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
}
