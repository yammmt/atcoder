use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        ss: [Chars; n],
    }

    let mut ans = ss;
    for i in (0..n - 1).rev() {
        for j in 1.. 2 * n - 1 {
            if ans[i][j] != '#' {
                continue;
            }

            if ans[i + 1][j - 1] == 'X'
                || ans[i + 1][j] == 'X'
                || ans[i + 1][j + 1] == 'X'
            {
                ans[i][j] = 'X';
            }
        }
    }

    for a in &ans {
        println!("{}", a.iter().collect::<String>());
    }
}
