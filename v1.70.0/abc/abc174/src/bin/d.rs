use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let r_cnt = s.iter().filter(|&&c| c == 'R').count();
    let ans = s.iter().take(r_cnt).filter(|&&c| c == 'W').count();
    println!("{ans}");
}
