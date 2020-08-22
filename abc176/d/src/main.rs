// -*- coding:utf-8-unix -*-

use proconio::input;
use proconio::marker::Chars;

// not work

fn dp(h: usize, w: usize, from_h: usize, from_w: usize, used_worp_num: u64, c: &Vec<Vec<char>>, worp_num: &mut Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    let mut updated_mass = vec!();

    for i in 1..6 {
        if from_h + i >= h {
            break;
        }

        if c[from_h + i][from_w] == '#' {
            break;
        }

        if used_worp_num < worp_num[from_h + i][from_w] {
            worp_num[from_h + i][from_w] = used_worp_num;
            updated_mass.push((from_h + i, from_w));
        }
    }
    for i in 1..6 {
        if i > from_h {
            break;
        }

        if c[from_h - i][from_w] == '#' {
            break;
        }

        if used_worp_num < worp_num[from_h - i][from_w] {
            worp_num[from_h - i][from_w] = used_worp_num;
            updated_mass.push((from_h - i, from_w));
        }
    }

    for i in 1..6 {
        if from_w + i >= w {
            break;
        }

        if c[from_h][from_w + i] == '#' {
            break;
        }

        if used_worp_num < worp_num[from_h][from_w + i] {
            worp_num[from_h][from_w + i] = used_worp_num;
            updated_mass.push((from_h, from_w + i));
        }
    }
    for i in 1..6 {
        if i > from_w {
            break;
        }

        if c[from_h][from_w - i] == '#' {
            break;
        }


        if used_worp_num < worp_num[from_h][from_w - i] {
            worp_num[from_h][from_w - i] = used_worp_num;
            updated_mass.push((from_h, from_w - i));
        }
    }

    for i in -2..3 {
        if from_h as isize + i >= h as isize {
            break;
        } else if from_h as isize + i < 0 {
            break;
        }

        for j in -2..3 {
            if from_w as isize + j >= w as isize {
                break;
            } else if from_w as isize + i < 0 {
                break;
            }

            if c[(from_h as isize + i) as usize][(from_w as isize + j) as usize] == '.' {
                if used_worp_num + 1 < worp_num[(from_h as isize + i) as usize][(from_w as isize + j) as usize] {
                    worp_num[(from_h as isize + i) as usize][(from_w as isize + j) as usize] = used_worp_num + 1;
                    updated_mass.push(((from_h as isize + i) as usize, (from_w as isize + j) as usize));
                }
            }
        }
    }

    // 更新のあった座標に対し再帰
    for (new_h, new_w) in updated_mass {
        dp(h, w, new_h, new_w, used_worp_num, &c, &mut worp_num);
    }

    worp_num.to_vec()
}

fn main() {
    input! {
        h: usize,
        w: usize,
        ch: usize,
        cw: usize,
        dh: usize,
        dw: usize,
        c: [Chars; h],
    }
    let mut worp_num = vec![vec![std::u64::MAX; 1000]; 1000];
    let worp_num = dp(h, w, ch, cw, 0, &c, &mut worp_num);

    if worp_num[dh][dw] == std::u64::MAX {
        println!("-1");
    } else {
        println!("{}", worp_num[dh][dw]);
    }
}
