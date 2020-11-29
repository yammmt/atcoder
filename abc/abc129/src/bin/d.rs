use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut from_left = vec![vec![0; w]; h];
    for i in 0..h {
        let mut cur = 0;
        for j in 0..w {
            match s[i][j] {
                '#' => cur = 0,
                '.' => {
                    from_left[i][j] = cur;
                    cur += 1;
                }
                _ => unreachable!(),
            }
        }
    }
    let mut from_right = vec![vec![0; w]; h];
    for i in 0..h {
        let mut cur = 0;
        for j in (0..w).rev() {
            match s[i][j] {
                '#' => cur = 0,
                '.' => {
                    from_right[i][j] = cur;
                    cur += 1;
                }
                _ => unreachable!(),
            }
        }
    }
    let mut from_up = vec![vec![0; w]; h];
    for j in 0..w {
        let mut cur = 0;
        for i in 0..h {
            match s[i][j] {
                '#' => cur = 0,
                '.' => {
                    from_up[i][j] = cur;
                    cur += 1;
                }
                _ => unreachable!(),
            }
        }
    }
    let mut from_down = vec![vec![0; w]; h];
    for j in 0..w {
        let mut cur = 0;
        for i in (0..h).rev() {
            match s[i][j] {
                '#' => cur = 0,
                '.' => {
                    from_down[i][j] = cur;
                    cur += 1;
                }
                _ => unreachable!(),
            }
        }
    }

    // '.' が一つ以上存在することが保証されている
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            ans = ans.max(
                from_left[i][j] + from_right[i][j] + from_up[i][j] + from_down[i][j]
            );
        }
    }
    println!("{}", ans + 1);
}
