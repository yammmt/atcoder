use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        n: usize,
        sn: [Chars; n],
    }
    let dir = [(1, 0), (0, 1), (1, 1), (1, -1)];

    let i_in_range = |i| {
        !(i < 0 || i >= n as isize)
    };

    for i in 0..n {
        for j in 0..n {
            let i_i = i as isize;
            let j_i = j as isize;
            // 6 マス調べて 4 マス以上塗られていればよい
            for d in &dir {
                let i6 = i_i + 5 * d.0;
                let j6 = j_i + 5 * d.1;
                if !i_in_range(i6) || !i_in_range(j6) {
                    continue;
                }

                let mut cnt = 0;
                for k in 0..6 {
                    let i_cur = (i_i + k as isize * d.0) as usize;
                    let j_cur = (j_i + k as isize * d.1) as usize;
                    if sn[i_cur][j_cur] == '#' {
                        cnt += 1;
                    }
                }

                if cnt >= 4 {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
}
