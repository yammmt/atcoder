use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut black_num_left = vec![0; n];
    let mut cur = 0;
    for i in 0..n {
        black_num_left[i] = cur;
        if s[i] == '#' {
            cur += 1;
        }
    }

    let mut white_num_right = vec![0; n];
    let mut cur = 0;
    for i in (0..n).rev() {
        white_num_right[i] = cur;
        if s[i] == '.' {
            cur += 1;
        }
    }

    let mut ans = n;
    for i in 0..n {
        ans = ans.min(black_num_left[i] + white_num_right[i]);
    }
    println!("{}", ans);
}
