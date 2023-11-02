// 重実装 本当に？順列一つにインデックス三つ使えばもう少し楽にならない？

use itertools::Itertools;
use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        n: usize,
        r: Chars,
        c: Chars,
    }

    for ap in (0..n).permutations(n) {
        let mut v = vec![vec!['.'; n]; n];
        for i in 0..n {
            v[i][ap[i]] = 'A';
        }

        'bp_loop: for bp in (0..n).permutations(n) {
            for i in 0..n {
                for j in 0..n {
                    if v[i][j] != 'A' {
                        v[i][j] = '.';
                    }
                }
            }

            for i in 0..n {
                if v[i][bp[i]] != '.' {
                    continue 'bp_loop;
                }

                v[i][bp[i]] = 'B';
            }

            'cp_loop: for cp in (0..n).permutations(n) {
                for i in 0..n {
                    for j in 0..n {
                        if v[i][j] != 'A' && v[i][j] != 'B' {
                            v[i][j] = '.';
                        }
                    }
                }

                for i in 0..n {
                    if v[i][cp[i]] != '.' {
                        continue 'cp_loop;
                    }

                    v[i][cp[i]] = 'C';
                }

                // r
                let mut r_pass = true;
                for i in 0..n {
                    for j in 0..n {
                        if v[i][j] != '.' {
                            if v[i][j] != r[i] {
                                r_pass = false;
                            }
                            break;
                        }
                    }
                }
                if !r_pass {
                    continue;
                }

                let mut c_pass = true;
                for i in 0..n {
                    for j in 0..n {
                        if v[j][i] != '.' {
                            if v[j][i] != c[i] {
                                c_pass = false;
                            }
                            break;
                        }
                    }
                }
                if c_pass {
                    println!("Yes");
                    for i in 0..n {
                        for j in 0..n {
                            print!("{}", v[i][j]);
                        }
                        println!();
                    }
                    return;
                }
            }
        }
    }

    println!("No");
}
