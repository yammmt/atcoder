// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [u64; n],
    }

    let mut psum = p.iter().take(k).sum::<u64>();
    let mut pmax = psum;
    let mut pstart = 0;
    let mut pstartnow = 1;
    while pstartnow + k - 1 < n {
        psum -= p[pstartnow - 1];
        psum += p[pstartnow + k - 1];
        if psum > pmax {
            pmax = psum;
            pstart = pstartnow;
        }
        pstartnow += 1;
    }

    let mut ans = 0.0f64;
    for i in pstart..pstart + k {
        let mut c = 0.0f64;
        for j in 1..p[i as usize] + 1 {
            c += j as f64 / p[i as usize] as f64;
        }
        ans += c;
    }
    println!("{}", ans);

    // 計算誤差で死んだ？
    // let mut vexpected = Vec::with_capacity(n);
    // for i in &p {
    //     let mut p = 0.0;
    //     if *i == 1 {
    //         p = 1.0;
    //     } else {
    //         for j in 1..*i + 1 {
    //             p += j as f64 / *i as f64;
    //         }
    //     }
    //     vexpected.push(p);
    // }
    // // println!("{:?}", vexpected);

    // let mut emax = 0.0f64;
    // let mut cpts = vexpected.iter().take(k).sum::<f64>();
    // // println!("{}", cpts);
    // let mut pstart = 1;
    // while pstart + k - 1 < n {
    //     // println!("ps: {}", pstart);
    //     cpts -= vexpected[pstart - 1];
    //     cpts += vexpected[pstart + k - 1];
    //     emax = emax.max(cpts);
    //     pstart += 1;
    // }
    // println!("{}", emax);
}
