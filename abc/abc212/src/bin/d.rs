// 事故 水色の三完をやたら見る戦犯

use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    // 最小値を PQ でまとめて管理すると値の更新が O(N^2) となり TLE
    // 操作 2 単位で配列を作ると O(N^2) となり TLE
    // 捨てる操作が辛い
    // セグ木を貼るだけ？でもこれ D 問題だしそれしかないならば事故回なので捨て
    // 逆から見る？結局最悪時に無理

    for _ in 0..q {
        input! {
            n: usize,
        }

        match n {
            1 => {}
            2 => {}
            3 => {}
            _ => unreachable!(),
        }
    }
}
