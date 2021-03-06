use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: Chars,
        k: usize,
    }
    let d = 1_000_000_007;

    // x 桁の数に x 個の数を出すなら x! - (x - 1)! 通り, 先頭が 0 であるものを引く
    // (x + 1) 桁の数に x 個の数を出すなら x 個の数の並べ方が x! 通り,
    // そのそれぞれに対して空いた 1 個の数の置き場所が (x + 1) 通りで
    // そこから先頭が 0 であるものと重複分を引けば良いのだがどうしたものか
    // さらには最後に端数分をなんとかする必要もある 今はお手上げ
}
