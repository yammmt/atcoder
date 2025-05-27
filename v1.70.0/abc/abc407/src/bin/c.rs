use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        mut s: Chars,
    }

    // ある一段階の文字列に対して B を押す回数は高々 10 回
    // 最終位の数字についてはボタンを押す回数は固定
    // 入力逆順にして貪欲しよう
    s.reverse();

    let push_a = s.len();
    let mut push_b = 0;
    for c in s {
        let c_begin = (push_b % 10) as u8;
        let c_end = (c as u8 - b'0') % 10;
        push_b += (((c_end + 10) - c_begin) % 10) as usize;
    }

    println!("{}", push_a + push_b);
}
