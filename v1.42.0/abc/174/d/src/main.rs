use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: u32,
        cn: Chars,
    }

    // 左側に赤い石を固める
    let reds = cn.iter().filter(|&c| *c == 'R').count();
    let ans = cn.iter().take(reds).filter(|&c| *c == 'W').count();
    println!("{}", ans);
}
