// 整数 N は 10 進数表記で 8^20 以下か質問投げたい (Yes だと思う)
// 128bit 整数型で受ければ文字列変換の例外が発生しなくなり楽

// RE: N は 8 進数表記であり入力段階で 1e19 以上となり得る
// WA: n = 0

use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! {
        n: Bytes,
        k: usize,
    }

    let mut eight = n
        .iter()
        .map(|&a| (a - b'0') as usize)
        .collect::<Vec<usize>>();

    for _ in 0..k {
        let mut ten = 0;
        // 8 -> 10
        let mut base = 1;
        for i in 0..eight.len() {
            ten += base * eight[eight.len() - i - 1];
            base *= 8;
        }

        // 10 -> 9
        let mut nine = vec![];
        while ten > 0 {
            nine.push(ten % 9);
            ten /= 9;
        }
        nine.reverse();
        if nine.is_empty() {
            nine.push(0);
        }

        // replace (-> 8)
        eight = vec![];
        for i in &nine {
            eight.push(match i {
                8 => 5,
                _ => *i,
            });
        }
    }

    for e in &eight {
        print!("{}", e);
    }
    println!();
}
