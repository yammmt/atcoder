use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
    }
    let mut problems = vec![n; m + 1];
    let mut scores = vec![vec![false; m + 1]; n + 1];
    for _ in 0..q {
        input! {
            a: usize,
        }

        if a == 1 {
            input! {
                b: usize,
            }
            let mut ans = 0;
            for i in 1..m + 1 {
                if scores[b][i] {
                    ans += problems[i];
                }
            }
            println!("{}", ans);
        } else {
            input! {
                b: usize,
                c: usize,
            }
            scores[b][c] = true;
            problems[c] -= 1;
        }
    }
}
