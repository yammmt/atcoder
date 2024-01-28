use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut cnt = vec![0; 26];
    for c in s {
        cnt[(c as u8 - b'a') as usize] += 1;
    }

    let mut ans_i = 0;
    let mut ans_cnt = 0;
    for (i, c) in cnt.iter().enumerate() {
        if *c > ans_cnt {
            ans_i = i;
            ans_cnt = *c;
        }
    }

    println!("{}", (b'a' + ans_i as u8) as char);
}
