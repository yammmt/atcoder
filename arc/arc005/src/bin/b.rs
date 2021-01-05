// 22min
// match で R も RU も同列に扱おうとして詰まった

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut x: isize,
        mut y: isize,
        mut w: Chars,
        // mut w: String,
        c99: [Chars; 9],
    }
    x -= 1;
    y -= 1;

    let mut ans = vec![];
    let mut dir = (0, 0);
    for _ in 0..4 {
        x += dir.0;
        y += dir.1;
        ans.push(c99[y as usize][x as usize]);

        dir = (0, 0);
        for i in 0..w.len() {
            match w[i] {
                'R' => {
                    if x == 8 {
                        dir.0 = -1;
                        w[i] = 'L';
                    } else {
                        dir.0 = 1;
                    }
                },
                'L' => {
                    if x == 0 {
                        dir.0 = 1;
                        w[i] = 'R';
                    } else {
                        dir.0 = -1;
                    }
                },
                'U' => {
                    if y == 0 {
                        dir.1 = 1;
                        w[i] = 'D';
                    } else {
                        dir.1 = -1;
                    }
                },
                'D' => {
                    if y == 8 {
                        dir.1 = -1;
                        w[i] = 'U';
                    } else {
                        dir.1 = 1;
                    }
                },
                _ => unreachable!(),
            }
        }
    }

    println!("{}", ans.iter().collect::<String>());
}
