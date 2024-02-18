use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        _n: usize,
        t: Chars,
        shw: [Chars; h],
    }

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if shw[i][j] == '#' {
                continue;
            }

            let mut pass = true;
            let mut i_cur = i;
            let mut j_cur = j;
            for c in &t {
                let (i_nxt, j_nxt) = match c {
                    'L' => (i_cur, j_cur.wrapping_add_signed(-1)),
                    'R' => (i_cur, j_cur + 1),
                    'U' => (i_cur.wrapping_add_signed(-1), j_cur),
                    'D' => (i_cur + 1, j_cur),
                    _ => unreachable!(),
                };

                if i_nxt >= h || j_nxt >= w || shw[i_nxt][j_nxt] == '#' {
                    pass = false;
                    break;
                }

                i_cur = i_nxt;
                j_cur = j_nxt;
            }

            if pass {
                ans += 1;
            }
        }
    }

    println!("{ans}");
}
