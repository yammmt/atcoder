use proconio::input;
// use std::collections::BTreeSet;

// an/bm を平衡二分探索木でもたせれば任意の a/b の出現回数は数えられる
// が、全 a/b に対して走らせると TLE
// 合計値を足し引きしようにも、毎回最大値を更新した場合に引きの部分で O(N or M) かかって詰む

// BTreeSet で要素の和取れたっけ

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        txy: [(usize, usize, usize); q],
    }
}
