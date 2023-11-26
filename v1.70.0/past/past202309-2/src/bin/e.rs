use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        x: Chars,
    }

    let mut cnt = vec![0; 10];
    for c in &x {
        cnt[c.to_digit(10).unwrap() as usize] += 1;
    }

    println!(
        "{}",
        if x == vec!['1'] {
            0
        } else if cnt[0] == x.len() - 1 && cnt[1] == 1 {
            x.len() - 1
        } else {
            x.len()
        }
    );
}
