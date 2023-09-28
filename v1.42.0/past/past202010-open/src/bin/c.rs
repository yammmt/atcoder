use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        m: usize,
        snm: [Chars; n],
    }

    for i in 0..n {
        for j in 0..m {
            let mut ans = 0;

            for ii in -1..=1 {
                let cur_i_i = i as isize + ii;
                if cur_i_i < 0 || cur_i_i >= n as isize {
                    continue;
                }

                let cur_i_u = cur_i_i as usize;
                for jj in -1..=1 {
                    let cur_j_i = j as isize + jj;
                    if cur_j_i < 0 || cur_j_i >= m as isize {
                        continue;
                    }

                    let cur_j_u = cur_j_i as usize;

                    if snm[cur_i_u][cur_j_u] == '#' {
                        ans += 1;
                    }
                }
            }

            print!("{}", ans);
        }
        println!();
    }
}
