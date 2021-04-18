use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut rgbnum = vec![0; 3];
    for c in &s {
        match *c {
            'R' => rgbnum[0] += 1,
            'G' => rgbnum[1] += 1,
            'B' => rgbnum[2] += 1,
            _ => unreachable!(),
        }
    }

    let mut ans = rgbnum.iter().product::<i64>();
    for i in 0..n {
        for j in i + 1..n {
            if s[i] == s[j] {
                continue;
            }

            let k = 2 * j - i;
            if k >= n {
                continue;
            }

            if s[i] != s[k] && s[j] != s[k] {
                ans -= 1;
            }
        }
    }

    println!("{}", ans);
}
