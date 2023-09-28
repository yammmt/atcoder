use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let mut ans = 0;
    for i in 0..n {
        // TODO: use default def
        let mut ss = s.clone();
        for x in 0..n {
            for y in 0..n {
                ss[x][y] = s[(x + i) % n][y];
            }
        }

        'x: for x in 0..n {
            for y in 0..n {
                if ss[x][y] != ss[y][x] {
                    break 'x;
                }

                if x == n - 1 && y == n - 1 {
                    ans += n;
                }
            }
        }
    }
    println!("{}", ans);
}
