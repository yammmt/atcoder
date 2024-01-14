use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    // 渦巻きになる順
    let dir = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let is_empty_area = |area: (isize, isize), ans: &Vec<Vec<usize>>| {
        !(area.0 < 0
            || area.1 < 0
            || area.0 >= n as isize
            || area.1 >= n as isize
            || ans[area.0 as usize][area.1 as usize] != 0)
    };

    let mut ans = vec![vec![0; n]; n];
    let mut cur = (0isize, 0isize);
    let mut di = 0;
    for i in 1..n*n {
        ans[cur.0 as usize][cur.1 as usize] = i;

        let dir_i = dir[di].0;
        let dir_j = dir[di].1;
        if is_empty_area((cur.0 + dir_i, cur.1 + dir_j), &ans) {
            cur = (cur.0 + dir_i, cur.1 + dir_j);
        } else {
            di = (di + 1) % dir.len();
            cur = (cur.0 + dir[di].0, cur.1 + dir[di].1);
        }
    }

    for i in 0..n {
        for j in 0..n {
            if i == n / 2 && j == n / 2 {
                print!("T");
            } else {
                print!("{}", ans[i][j]);
            }
            if j != n - 1 {
                print!(" ");
            }
        }
        println!();
    }
}
