use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;
use superslice::Ext;

fn is_palindrome(s: &[char], k: usize, begin_i: usize) -> bool {
    for i in 0..k / 2 {
        if s[begin_i + i] != s[begin_i + k - i - 1] {
            return false;
        }
    }

    true
}

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        mut s: Chars,
    }
    s.sort_unstable();

    let mut ans = 0;
    // `while` ループでブロック内を継続条件として使う.
    // `while s.next_permutation()` だと初手の判定が漏れてしまうが
    // これだと初手は `s` の更新が入らないのでうまくいく.
    while {
        let mut passes = true;
        // i+k-1 < n <=> i < n-k-1 <=> i <= n-k
        for i in 0..=n - k {
            if is_palindrome(&s, k, i) {
                passes = false;
                break;
            }
        }

        if passes {
            ans += 1;
        }

        s.next_permutation()
    } {}

    println!("{ans}");
}
