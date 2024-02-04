// 嫌い 問題の読み辛さや実装量は 250 点問題じゃない 上向きってどこやねん

use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
    }
    let dir = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    let mut grid = vec![vec!['.'; w]; h];
    let mut tkhs = (0, 0);
    let mut di_prev = 0;

    for _ in 0..n {
        let mut di;
        if grid[tkhs.0][tkhs.1] == '.' {
            grid[tkhs.0][tkhs.1] = '#';
            di = (di_prev + 1) % dir.len();
        } else {
            grid[tkhs.0][tkhs.1] = '.';
            di = (di_prev + dir.len() - 1) % dir.len();
        }

        let x_nxt = (tkhs.0 as isize + h as isize + dir[di].0) as usize % h;
        let y_nxt = (tkhs.1 as isize + w as isize + dir[di].1) as usize % w;
        tkhs = (x_nxt, y_nxt);
        di_prev = di;
    }

    for i in 0..h {
        for j in 0..w {
            print!("{}", grid[i][j]);
        }
        println!();
    }
}
