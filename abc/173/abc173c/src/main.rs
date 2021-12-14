use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        c: [Chars; h],
    }

    let mut ans = 0;
    for ih in 0..2u32.pow(h as u32) {
        let mut selected_h = vec![false; h];
        let mut cur_ih = ih;
        for ii in 0..h {
            if cur_ih % 2 == 1 {
                selected_h[ii] = true;
            }
            cur_ih /= 2;
        }

        for jw in 0..2u32.pow(w as u32) {
            let mut selected_w = vec![false; w];
            let mut cur_jw = jw;
            for jj in 0..w {
                if cur_jw % 2 == 1 {
                    selected_w[jj] = true;
                }
                cur_jw /= 2;
            }

            let mut cur = 0;
            for i in 0..h {
                if selected_h[i] {
                    continue;
                }

                for j in 0..w {
                    if selected_w[j] {
                        continue;
                    }

                    if c[i][j] == '#' {
                        cur += 1;
                    }
                }
            }

            if cur == k {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
