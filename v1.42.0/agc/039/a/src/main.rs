// -*- coding:utf-8-unix -*-

use proconio::input;

// 100 分で解けると 960 くらい

fn main() {
    input! {
        s: String,
        k: usize,
    }

    let vc = s.chars().collect::<Vec<char>>();
    let mut ans = 0;
    let mut current = 0;
    if k == 1 {
        // 一周見て終わり
        for i in 0..vc.len() {
            if i == 0 {
                current += 1;
                continue;
            }

            if vc[i] == vc[i - 1] {
                current += 1;
            } else {
                ans += current / 2;
                current = 1;
            }
            if i == vc.len() - 1 {
                ans += current / 2;
            }
        }
    } else {
        // 二周見る
        let vcc = vc.repeat(2);
        for i in 0..vcc.len() {
            if i == 0 {
                current += 1;
                continue;
            }

            if vcc[i] == vcc[i - 1] {
                current += 1;
            } else {
                ans += current / 2;
                current = 1;
            }
            if i == vcc.len() - 1 {
                ans += current / 2;
            }
        }
        if current == vcc.len() {
            // すべて同じ文字なら一字おきに置き換えるだけ
            if k % 2 == 0 {
                println!("{}", ans * (k / 2));
            } else {
                println!("{}", ans * (k / 2) + ans / 2);
            }
            return;
        }

        let two_cyc = ans;
        // 一回目の文字列と二回目 (以降) の文字列で置き換え回数が違う場合、その差は明らかに一回
        // つまりは二周見た場合に置き換え回数が奇数回になっているか否かで場合分けする
        if two_cyc % 2 == 0 {
            if k % 2 == 0 {
                ans *= k / 2;
            } else {
                ans *= k / 2;
                ans += two_cyc / 2;
            }
        } else {
            // "aaabaaa" あたりを例として紙に書き出してみると良いかも
            let first_cyc = two_cyc / 2;
            let other_cyc = first_cyc + 1;
            ans = (k - 1) * other_cyc + first_cyc;
        }
    }
    println!("{}", ans);
}
