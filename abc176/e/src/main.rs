// -*- coding:utf-8-unix -*-

// TLE

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        m: usize,
    }

    let mut vh = Vec::with_capacity(h);
    let mut vw = Vec::with_capacity(w);
    for _ in 0..h {
        vh.push(0);
    }
    for _ in 0..w {
        vw.push(0);
    }
    let mut mass = Vec::with_capacity(m);
    // println!("len: {} {}", vh.len(), vw.len());

    for _ in 0..m {
        input! {
            hn: usize,
            wn: usize,
        }
        // println!("hn, wn: {}, {}", hn, wn);
        vh[hn - 1] += 1;
        vw[wn - 1] += 1;
        mass.push((hn - 1, wn - 1));
    }

    // 行最大 + 列最大 に -1 するか否か
    // 行最大 + 列最大が複数存在する可能性
    let mut hmax = 0;
    let mut hidx_max = vec!();
    for i in 0..vh.len() {
        if hmax < vh[i] {
            hmax = vh[i];
            hidx_max.clear();
            hidx_max.push(i);
        } else if hmax == vh[i] {
            hidx_max.push(i);
        }
    }

    let mut wmax = 0;
    let mut widx_max = vec!();
    for i in 0..vw.len() {
        if wmax < vw[i] {
            wmax = vw[i];
            widx_max.clear();
            widx_max.push(i);
        } else if wmax == vw[i] {
            widx_max.push(i);
        }
    }
    let ans = hmax + wmax;

    // h/w の idx に対してループ回すと O(n^2) で TLE
    let mut p = 0;
    for &(i, j) in &mass {
        if vh[i] == hmax && vw[j] == wmax {
            p += 1;
        }
    }
    if p == hidx_max.len() * widx_max.len() {
        println!("{}", ans - 1);
    } else {
        println!("{}", ans);
    }
}
