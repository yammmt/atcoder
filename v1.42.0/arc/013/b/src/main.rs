// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        c: usize,
    }
    let mut n_max = 0;
    let mut m_max = 0;
    let mut l_max = 0;
    for _ in 0..c {
        input! {
            n: usize,
            m: usize,
            l: usize,
        }
        let mut v = vec![n, m, l];
        v.sort();
        n_max = n_max.max(v[0]);
        m_max = m_max.max(v[1]);
        l_max = l_max.max(v[2]);
    }
    println!("{}", n_max * m_max * l_max);
}
