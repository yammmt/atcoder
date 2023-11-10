use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        n: usize,
        sn: [Chars; n],
    }

    let mut ans = isize::MAX;
    for i in 0..=9 {
        let mut cnt = vec![0; 10];
        for s in &sn {
            for j in 0..10 {
                if s[j] == std::char::from_digit(i, 10).unwrap() {
                    cnt[j] += 1;
                    break;
                }
            }
        }

        let mut cur = 0;
        for (j, c) in cnt.iter().enumerate() {
            if *c == 0 {
                continue;
            }

            cur = cur.max(j as isize + (*c as isize - 1).max(0) * 10);
        }
        ans = ans.min(cur);
    }

    println!("{ans}");
}
