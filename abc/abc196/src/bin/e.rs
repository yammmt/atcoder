use proconio::input;
use std::collections::VecDeque;

const NO_MAX_MIN_APPLIED: i64 = std::i64::MAX / 2;

fn main() {
    input! {
        n: usize,
        atn: [(i64, usize); n],
        q: usize,
        xq: [usize; q],
    }

    let mut inputs = vec![];
    for (i, x) in xq.iter().enumerate() {
        inputs.push((*x, i));
    }
    inputs.sort_unstable();

    // 加算分を別に記憶しておけば max/min は一括で判定を取れるし, 操作によって値の大小は変化しない
    // t = 1: 全範囲への加算分を足す
    // t = 2: a 以下の値をすべて x に置き換えて加算分をリセットする
    // t = 3: a 以上の値をすべて x に置き換えて加算分をリセットする
    // "加算分をリセット": 愚直にまわすと O(n) となり TLE, 繰り返し実行されるので単純ないもす法ではだめ
    // 区間への操作となるとセグ木を思い浮かべるが E で出すか？
    // 区間がどちらかの端を含むおかげで加算値と範囲を両端キューに突っ込めばいけそう
    let mut add_dq = VecDeque::new();
    add_dq.push_back((0, (0, n)));
    let mut const_dq = VecDeque::new();
    const_dq.push_back((NO_MAX_MIN_APPLIED, (0, n)));

    let mut ans = vec![0; n];
    // 実際の答えをまとめて計算する
    ans.iter().for_each(|a| println!("{}", a));
}
