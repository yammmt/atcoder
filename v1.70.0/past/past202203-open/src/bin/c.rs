use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        a: Chars,
    }

    // 77,777
    // 9,982,443,539,982,448,531,000,000,007
    // 3 桁ごとに区切るたびに英字をインクリメント

    let mut digits = a.len() as isize;
    let mut three_increments = 0;
    while digits > 3 {
        three_increments += 1;
        digits -= 3;
    }

    for i in 0..digits as usize {
        print!("{}", a[i]);
    }
    println!("{}", (b'a' + three_increments - 1) as char);
}
