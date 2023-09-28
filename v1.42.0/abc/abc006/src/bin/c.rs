// WA: 探索範囲が [0, n) になっていた + 大人だけで構成した場合の出力対象

use proconio::input;

fn main() {
    input! {
        n: i32,
        m: i32,
    }

    let all_adults = 2 * n;
    let diff = m - all_adults;
    // println!("all adults: {}", all_adults);
    // println!("diff: {}", diff);
    if diff == 0 {
        println!("{} 0 0", n);
        return;
    }

    for olds in 0..n + 1 {
        // println!("olds: {}", olds);
        // println!("diff - olds: {}", diff - olds);
        if diff - olds == 0 {
            println!("{} {} 0", n - olds, olds);
            return;
        }

        if (diff - olds) % 2 == 0 {
            let children = (diff - olds) / 2;
            let adults = n - olds - children;
            if adults >= 0 && olds >= 0 && children >= 0 && adults + olds + children == n && 2 * adults + 3 * olds + 4 * children == m {
                println!("{} {} {}", adults, olds, children);
                return;
            }
        }
    }
    println!("-1 -1 -1");
}
