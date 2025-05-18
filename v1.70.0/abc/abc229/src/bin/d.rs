use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        s: Chars,
        k: usize,
    }
    let n = s.len();

    let mut ans = 0;
    let mut right = 0;
    let mut dot_count = 0;
    for left in 0..n {
        // テストケースに k=0 なくないか？
        // => そんなことはなく, 最適化で通っているっぽい

        // これないと一文字目 "." かつ k=0 で動けなくなる
        if right < left {
            right = left;
        }

        while right < n && (s[right] == 'X' || dot_count < k) {
            if s[right] == '.' {
                dot_count += 1;
            }
            right += 1;
        }

        ans = ans.max(right - left);
        if s[left] == '.' && dot_count > 0 {
            dot_count -= 1;
        }
    }

    println!("{ans}");
}
