// :fu: 問題以前に？まったく頭がまわらないのでやり直す

// 曰く桁 DP
// https://drken1215.hatenablog.com/entry/2020/02/09/225300

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        nn: Chars,
        k: usize,
    }
    let n = nn.iter().map(|a| (*a as u8 - b'0') as usize).collect::<Vec<usize>>();
}
