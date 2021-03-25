use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: u32,
        c: Chars,
    }
    let red_num = c.iter().filter(|&s| *s == 'R').count();
    println!("{}", c.iter().take(red_num).filter(|&s| *s == 'W').count());
}
