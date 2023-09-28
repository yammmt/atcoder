use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        x: usize,
        y: usize,
        sh: [Chars; h],
    }
    let dir = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    let mut ans = 1; //self
    for d in &dir {
        let mut cur = (x as isize - 1 + d.0, y as isize - 1 + d.1);
        while cur.0 >= 0 && cur.1 >= 0 && cur.0 < h as isize && cur.1 < w as isize {
            if sh[cur.0 as usize][cur.1 as usize] == '#' {
                break;
            }

            ans += 1;
            cur = (cur.0 + d.0, cur.1 + d.1);
        }
    }

    println!("{}", ans);
}
