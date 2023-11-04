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

    // 枝刈りを入れられるけれど, 競技だし計算量を減らすより実装で楽した方がよいとして
    for ai in (0..n).permutations(n) {
        for bj in (0..n).permutations(n) {
            'outer: for ck in (0..n).permutations(n) {
                let mut v = vec![vec!['.'; n]; n];
                for i in 0..n {
                    v[i][ai[i]] = 'A';
                }
                for i in 0..n {
                    if v[i][bj[i]] != '.' {
                        continue 'outer;
                    }

                    v[i][bj[i]] = 'B';
                }
                for i in 0..n {
                    if v[i][ck[i]] != '.' {
                        continue 'outer;
                    }

                    v[i][ck[i]] = 'C';
                }

                let mut v_col = vec![];
                for i in 0..n {
                    for j in 0..n {
                        if v[i][j] != '.' {
                            v_col.push(v[i][j]);
                            break;
                        }
                    }
                }
                if v_col != r {
                    continue;
                }

                let mut v_row = vec![];
                for i in 0..n {
                    for j in 0..n {
                        if v[j][i] != '.' {
                            v_row.push(v[j][i]);
                            break;
                        }
                    }
                }
                if v_row != c {
                    continue;
                }

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

    println!("No");
}
