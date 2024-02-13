use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        snm: [Chars; n];
    }

    for i in 0..n {
        if i + 8 >= n {
            break;
        }

        for j in 0..m {
            if j + 8 >= m {
                break;
            }

            let mut black_pass = true;
            'k_loop: for k in 0..2 {
                for l in 0..2 {
                    if snm[i+k][j+l] != '#' || snm[i+8-k][j+8-l] != '#' {
                        black_pass = false;
                        break 'k_loop;
                    }
                }
            }
            if !black_pass {
                continue;
            }

            let mut white_pass = true;
            for k in 0..2 {
                if snm[i+k][j+3] != '.' || snm[i+3][j+k] != '.' || snm[i+8-k][j-3] != '.' || snm[i-3][j+8-k] != '.' {
                    white_pass = false;
                    break;
                }
            }
            if snm[i+3][j+3] != '.' || snm[i+8-3][j+8-3] != '.' {
                white_pass = false;
            }

            if white_pass && black_pass {
                println!("{} {}", i+1, j+1);
            }
        }
    }
}
