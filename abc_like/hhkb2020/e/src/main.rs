// -*- coding:utf-8-unix -*-

// WA
// 2000x2000 全マス置ける場合に地球が爆発する計算量になるのではと思いきや TLE ではない
// `2u64.pow()` がオーバーフローしたのだろう

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
    }
    let mut s = vec![];
    let mut k = vec![];
    for i in 0..h {
        input! {
            st: String,
        }
        let vc = st.chars().collect::<Vec<char>>();
        for j in 0..vc.len() {
            if vc[j] == '.' {
                k.push((i as usize, j as usize));
            }
        }
        s.push(vc);
    }
    // println!("{:?}", k);
    let divisor = 1_000_000_007;
    let mut ans = 0u64;
    for bit_row in 0..2u64.pow(k.len() as u32) {
        let mut selected = vec![];
        for i in 0..k.len() {
            if bit_row & (1 << i) > 0 {
                selected.push(i);
            }
        }
        // println!("selected: {:?}", selected);

        let mut mass = s.clone();
        for i in &selected {
            // println!("i: {:?}", i);
            // println!("{:?}", k[*i as usize]);
            // self
            if mass[k[*i as usize].0][k[*i as usize].1] == '.' {
                mass[k[*i as usize].0][k[*i as usize].1] = 'o';
                ans = (ans + 1) % divisor;
            }
            // h 負方向
            for j in (0..k[*i as usize].0).rev() {
                if mass[j][k[*i as usize].1] == '#' {
                    break;
                } else if mass[j][k[*i as usize].1] == '.' {
                    mass[j][k[*i as usize].1] = 'o';
                    ans = (ans + 1) % divisor;
                }
            }
            // h 正方向
            for j in k[*i as usize].0..h {
                if mass[j][k[*i as usize].1] == '#' {
                    break;
                } else if mass[j][k[*i as usize].1] == '.' {
                    mass[j][k[*i as usize].1] = 'o';
                    ans = (ans + 1) % divisor;
                }
            }
            // w 負方向
            for j in (0..k[*i as usize].1).rev() {
                if mass[k[*i as usize].0][j] == '#' {
                    break;
                } else if mass[k[*i as usize].0][j] == '.' {
                    mass[k[*i as usize].0][j] = 'o';
                    ans = (ans + 1) % divisor;
                }
            }
            // w 正方向
            for j in k[*i as usize].1..w {
                if mass[k[*i as usize].0][j] == '#' {
                    break;
                } else if mass[k[*i as usize].0][j] == '.' {
                    mass[k[*i as usize].0][j] = 'o';
                    ans = (ans + 1) % divisor;
                }
            }
            // println!("{:?}", mass);
        }
        // println!("{:?}", mass);
    }

    println!("{}", ans);
}
