use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut s: Chars,
    }

    // 操作回数は高々 50 回
    // 愚直シミュレーションだと 2^50 回の試行となり TLE
    // - 左端にいる忍者は 1 に達するまで A をする
    // - 右端にいる忍者は N に達するまで B をする
    // - これら操作完了後に中間域が空いていたら近い側の操作で埋める
    //   - これ貪欲でよい？

    let mut ans_x = 0;
    while s[0] != '#' {
        ans_x += 1;
        for i in 0..n - 1 {
            if s[i + 1] == '#' {
                s[i] = '#'
            }
        }
    }

    let mut ans_y = 0;
    while s[n - 1] != '#' {
        ans_y += 1;
        for i in (1..n).rev() {
            if s[i - 1] == '#' {
                s[i] = '#'
            }
        }
    }

    for i in 1..n - 1 {
        if s[i] == '#' {
            continue;
        }

        let mut op_num = 1;
        loop {
            let i_a = (i + op_num).min(n - 1);
            if s[i_a] == '#' {
                ans_x += op_num;
                for _ in 0..op_num {
                    for j in 0..n - 1 {
                        if s[j + 1] == '#' {
                            s[j] = '#';
                        }
                    }
                }
                break;
            }

            let i_b = i - op_num.min(i);
            if s[i_b] == '#' {
                ans_y += op_num;
                for _ in 0..op_num {
                    for j in (1..n).rev() {
                        if s[j - 1] == '#' {
                            s[j] = '#';
                        }
                    }
                }
                break;
            }

            op_num += 1;
        }
    }

    println!("{ans_x} {ans_y}");
}
