// NOT WORK 最小カット問題

use proconio::input;

fn main() {
    input! {
        n: usize,
        g: usize,
        e: usize,
        pg: [usize; g],
        abe: [(usize, usize); e],
    }

    // 辺/パスワードを操作するか否かで 2^G * 2^E 通り計算すればわかるが TLE
    // 最悪 G 人のパスワードをすべて変えてやれば良い
    // 例 3 が良い
    // 0 起点で Union-Find して p が含まれる連結成分を落とす？
    // 辺を落とした方が良い条件はその辺を通らなければならない頂点が複数ある場合
    // p から 0 に至る頂点を全探索して重複あったら (0 に近いものから) 消す, 最後にあまった頂点を落とす？
    // 実装重いけれど計算量は間に合う範囲
}