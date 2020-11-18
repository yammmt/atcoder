// 10min

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        shw: [Chars; h],
    }

    let d = [(-1isize, 0isize), (1isize, 0isize), (0isize, -1isize), (0isize, 1isize)];
    for i in 0..h {
        for j in 0..w {
            if shw[i][j] == '.' {
                continue;
            }

            let mut passed = false;
            let base = (i as isize, j as isize);
            for k in &d {
                let cur = ((base.0 + k.0), (base.1 + k.1));
                if cur.0 >= 0 && cur.0 < h as isize && cur.1 >=0 && cur.1 < w as isize && shw[cur.0 as usize][cur.1 as usize] == '#' {
                    passed = true;
                    break;
                }
            }
            if !passed {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
