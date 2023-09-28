// -*- coding:utf-8-unix -*-

// https://drken1215.hatenablog.com/entry/2019/06/10/143300

// 各マスに対する上下左右の範囲を毎度検索すると O(HHW + HWW) くらいになり TLE

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
    }
    let mut s = vec![];
    let mut empty_spaces = 0u64;
    for _ in 0..h {
        input! {
            st: String,
        }
        let vc = st.chars().collect::<Vec<char>>();
        empty_spaces += vc.iter().filter(|&c| *c == '.').count() as u64;
        s.push(vc);
    }
    // println!("{:?}", s);

    let mut left = vec![vec![0u64; w]; h];
    let mut right = vec![vec![0u64; w]; h];
    let mut up = vec![vec![0u64; w]; h];
    let mut down = vec![vec![0u64; w]; h];
    for i in 0..h {
        let mut cur = 0;
        for j in 0..w {
            if s[i][j] == '.' {
                cur += 1;
                left[i][j] = cur;
            } else {
                cur = 0;
            }
        }
    }
    for i in 0..h {
        let mut cur = 0;
        for j in (0..w).rev() {
            if s[i][j] == '.' {
                cur += 1;
                right[i][j] = cur;
            } else {
                cur = 0;
            }
        }
    }
    for i in 0..w {
        let mut cur = 0;
        for j in 0..h {
            if s[j][i] == '.' {
                cur += 1;
                down[j][i] = cur;
            } else {
                cur = 0;
            }
        }
    }
    for i in 0..w {
        let mut cur = 0;
        for j in (0..h).rev() {
            if s[j][i] == '.' {
                cur += 1;
                up[j][i] = cur;
            } else {
                cur = 0;
            }
        }
    }

    let divisor = 1_000_000_007u64;
    let mut twon = Vec::with_capacity(h * w + 1);
    // 繰り返し二乗法だと高速だが h/w は高々 2,000 乗算なので間に合う
    for i in 0..h * w + 1 {
        if i == 0 {
            twon.push(1);
        } else {
            twon.push((twon[i - 1] * 2) % divisor);
        }
    }

    let mut ans = 0u64;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }

            // 自身を点灯する n 個のマスのうちどれかが点灯すれば良い
            // 全消灯のパターンを引くための 2^n - 1
            // 自身を点灯させない m 個のマスは点灯消灯どちらでも良いので単純に 2^m
            let light_ptrn = left[i][j] + right[i][j] + up[i][j] + down[i][j] - 3;
            ans = (ans + (((twon[light_ptrn as usize] + divisor - 1) % divisor) * twon[(empty_spaces - light_ptrn) as usize]) % divisor) % divisor;
        }
    }

    println!("{}", ans);
}
