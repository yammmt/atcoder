// 46min 3WA (29min-)
// WA: 初期パターンミスと条件一致を最後だけしか見ていなかった
// bool 型で実装したのは偉い

use proconio::input;

fn solve(n: usize, f: bool, l: bool, s: &[char]) -> bool {
    // S if true
    let mut ans = vec![true; n];
    ans[0] = f;
    ans[n - 1] = l;

    for i in 0..n - 1 {
        // println!("{}, {}", ans[i], s[i]);
        ans[i + 1] = if ans[i] {
            // sheep
            if s[i] == 'o' {
                ans[(i + n - 1) % n]
            } else {
                !ans[(i + n - 1) % n]
            }
        } else {
            // wolf
            if s[i] == 'o' {
                !ans[(i + n - 1) % n]
            } else {
                ans[(i + n - 1) % n]
            }
        };
    }
    // println!("{:?}", ans);

    for i in 0..n {
        let left = ans[(i + n - 1) % n];
        let right = ans[(i + 1) % n];

        if (ans[i] && s[i] == 'o' && left != right)
            || (ans[i] && s[i] == 'x' && left == right)
            || (!ans[i] && s[i] == 'o' && left == right)
            || (!ans[i] && s[i] == 'x' && left != right)
        {
            return false;
        }
    }

    // pass
    for c in &ans {
        print!("{}", if *c { 'S' } else { 'W' });
    }
    println!();

    true
}

fn main() {
    input! {
        n: usize,
        s: String,
    }
    let vc = s.chars().collect::<Vec<char>>();
    let params = [(true, true), (true, false), (false, true), (false, false)];
    for p in &params {
        if solve(n, p.0, p.1, &vc) {
            return;
        }
    }

    println!("-1");
}
